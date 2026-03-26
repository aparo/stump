import os
import re
from pathlib import Path

ROOT_DIR = Path(__file__).parent.parent
ENTITY_DIR = ROOT_DIR / "crates" / "models" / "src" / "entity"


def to_snake_case(name):
    s1 = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


def main():
    tables = {}
    mod_to_table = {}

    for filename in os.listdir(ENTITY_DIR):
        if filename == "mod.rs" or not filename.endswith(".rs"):
            continue

        with open(os.path.join(ENTITY_DIR, filename), "r") as f:
            content = f.read()

        table_match = re.search(
            r'#\[sea_orm\(\s*table_name\s*=\s*"([^"]+)"\s*\)\]', content
        )
        if not table_match:
            continue
        table_name = table_match.group(1)
        mod_name = filename[:-3]
        mod_to_table[mod_name] = table_name

        struct_match = re.search(r"pub struct Model \{(.*?)\}", content, re.DOTALL)
        if not struct_match:
            continue

        fields_str = struct_match.group(1)
        fields = []
        lines = fields_str.split("\n")
        is_pk = False
        nullable = False

        for line in lines:
            line = line.strip()
            if not line:
                continue
            if line.startswith("//"):
                continue

            if line.startswith("#[sea_orm"):
                if "primary_key" in line:
                    is_pk = True
                if "nullable" in line:
                    nullable = True
                continue

            if line.startswith("pub "):
                parts = line.replace("pub ", "").split(":")
                if len(parts) >= 2:
                    col_name = parts[0].strip()
                    col_type = parts[1].split(",")[0].strip()

                    if col_type.startswith("Option<"):
                        nullable = True
                        col_type = col_type[7:-1]

                    fields.append(
                        {
                            "name": col_name,
                            "type": col_type,
                            "is_pk": is_pk,
                            "nullable": nullable,
                        }
                    )
                is_pk = False
                nullable = False

        relations = []
        rel_enum_match = re.search(r"pub enum Relation \{(.*?)\}", content, re.DOTALL)
        if rel_enum_match:
            rel_lines = rel_enum_match.group(1)
            blocks = re.split(r"#\[sea_orm\(", rel_lines)
            for block in blocks:
                if "belongs_to" in block:
                    bt_match = re.search(
                        r'belongs_to\s*=\s*".*?::(\w+)::Entity"', block
                    )
                    frm_match = re.search(r'from\s*=\s*"Column::(\w+)"', block)
                    to_match = re.search(r'to\s*=\s*".*?::Column::(\w+)"', block)
                    if bt_match and frm_match and to_match:
                        relations.append(
                            {
                                "target_mod": bt_match.group(1),
                                "from": to_snake_case(frm_match.group(1)),
                                "to": to_snake_case(to_match.group(1)),
                            }
                        )

        tables[table_name] = {
            "module": mod_name,
            "fields": fields,
            "relations": relations,
        }

    # Generate Mermaid
    mmd = ["```mermaid", "erDiagram"]
    for table_name, data in tables.items():
        mmd.append(f"    {table_name} {{")
        fk_cols = set([r["from"] for r in data["relations"]])
        for f in data["fields"]:
            mods = []
            if f["is_pk"]:
                mods.append("PK")
            if f["name"] in fk_cols:
                mods.append("FK")
            if f["nullable"]:
                mods.append('"nullable"')
            mods_str = " ".join(mods)

            # Escape types if they have inner braces or so
            safe_type = f["type"].replace(" ", "_").replace("<", "_").replace(">", "")
            mmd.append(f"        {safe_type} {f['name']} {mods_str}")
        mmd.append("    }")

    for table_name, data in tables.items():
        for r in data["relations"]:
            target_table = mod_to_table.get(r["target_mod"])
            if target_table:
                # Is FK nullable?
                is_nullable = False
                for f in data["fields"]:
                    if f["name"] == r["from"]:
                        is_nullable = f["nullable"]

                # Zero-or-one if nullable, else exactly-one
                left = "|o" if is_nullable else "||"
                mmd.append(
                    f'    {target_table} {left}--o{{ {table_name} : "{r["from"]} -> {r["to"]}"'
                )

    mmd.append("```\n")

    os.makedirs("docs", exist_ok=True)
    with open("docs/db_schema.md", "w") as f:
        f.write("# Database Schema\n\n")
        f.write("\n".join(mmd))
    print(f"Generated docs/db_schema.md with {len(tables)} tables.")


if __name__ == "__main__":
    main()
