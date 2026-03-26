# Database Schema

```mermaid
erDiagram
    favorite_series {
        Uuid user_id PKFK
        Uuid series_id PKFK
        DateTimeWithTimeZone favorited_at
    }
    media_tags {
        i32 id PK
        Uuid media_id FKey
        i32 tag_id FKey
    }
    server_config {
        i32 id PK
        String public_url Null
        bool initial_wal_setup_complete
        String encryption_key Null
    }
    smart_list_access_rules {
        i32 id PK
        SmartListAccessRole role
        Uuid user_id FKey
        Uuid smart_list_id FKey
    }
    notifiers {
        i32 id PK
        String type
        Vec_u8 config
    }
    library_exclusions {
        i32 id PK
        Uuid user_id FKey
        Uuid library_id FKey
    }
    bookmarks {
        Uuid id PK
        String preview_content Null
        ReadiumLocator locator Null
        String epubcfi Null
        i32 page Null
        Uuid media_id FKey
        Uuid user_id FKey
        DateTimeUtc created_at
    }
    book_club_invitations {
        Uuid id PK
        BookClubMemberRole role
        Uuid user_id FKey
        Uuid book_club_id FKey
    }
    book_clubs {
        Uuid id PK
        String name
        String slug
        String description Null
        bool is_private
        BookClubMemberRoleSpec member_role_spec Null
        DateTimeWithTimeZone created_at
        String emoji
    }
    registered_reading_devices {
        Uuid id PK
        String name
        String kind
    }
    series_metadata {
        Uuid series_id PKFK
        i32 age_rating Null
        String characters Null
        String booktype Null
        CollectedItems collects Null
        i32 comicid Null
        String comic_image Null
        String description_formatted Null
        String genres Null
        String imprint Null
        String links Null
        String meta_type Null
        String publication_run Null
        String publisher Null
        String status Null
        String summary Null
        String title Null
        i32 total_issues Null
        i32 volume Null
        String writers Null
        i32 year
    }
    finished_reading_sessions {
        i32 id PK
        DateTimeWithTimeZone started_at
        DateTimeWithTimeZone completed_at
        Uuid media_id FKey
        Uuid user_id FKey
        Uuid device_id FKNull
        i64 elapsed_seconds Null
    }
    users {
        Uuid id PK
        String username
        String hashed_password
        bool is_server_owner
        String avatar_path Null
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone deleted_at Null
        bool is_locked
        i32 max_sessions_allowed Null
        String permissions Null
        i32 user_preferences_id FKNull
        String oidc_issuer_id Null
        String oidc_email
    }
    reading_list_rules {
        i32 id PK
        i32 role
        String user_id
        String reading_list_id FK
    }
    book_club_member_favorite_books {
        Uuid id PK
        String title Null
        String author Null
        String url Null
        String notes Null
        Uuid member_id FKey
        Uuid book_id FKNull
        StringNull image_url
    }
    book_club_book_suggestion_likes {
        i32 id PK
        DateTimeWithTimeZone timestamp
        Uuid liked_by_id FKey
        Uuid suggestion_id FKey
    }
    refresh_tokens {
        String id PK
        Uuid user_id FKey
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone expires_at NotNull
    }
    smart_lists {
        Uuid id PK
        Uuid creator_id FKey
        String name
        String description
        Vec_u8 filters
        SmartListJoiner joiner
        SmartListGrouping default_grouping
        EntityVisibility visibility NotNull
    }
    smart_list_views {
        i32 id PK
        String name
        Uuid list_id FKey
        Vec_u8 data NotNull
    }
    custom_emojis {
        i32 id PK
        String name
        bool is_animated
        String file_extension
        DateTimeWithTimeZone created_at
        Uuid created_by_id FK
    }
    book_club_discussion_message {
        Uuid id PK
        String content
        DateTimeWithTimeZone timestamp
        DateTimeWithTimeZone edited_at Null
        bool is_pinned_message
        DateTimeWithTimeZone deleted_at Null
        Uuid parent_message_id Null
        Uuid reply_to_message_id Null
        Uuid discussion_id FKey
        Uuid member_id FKNull
        Uuid book_club_id FK
    }
    media {
        Uuid id PK
        String name
        i64 size
        String extension
        i32 pages
        DateTimeWithTimeZone updated_at Null
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone modified_at Null
        String hash Null
        String koreader_hash Null
        String path
        FileStatus status
        ImageMetadata thumbnail_meta Null
        String thumbnail_path Null
        Uuid series_id FKNull
        DateTimeWithTimeZone deleted_at Null
    }
    user_login_activity {
        i64 id PK
        String ip_address
        String user_agent
        bool authentication_successful
        DateTimeWithTimeZone timestamp
        Uuid user_id FK
    }
    api_keys {
        i32 id PK
        String name
        String short_token
        String long_token_hash
        Uuid user_id FKey
        APIKeyPermissions permissions Null
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone last_used_at Null
        DateTimeWithTimeZone expires_at NotNull
    }
    jobs {
        Uuid id PK
        String name
        String description Null
        JobStatus status
        Vec_u8 save_state Null
        Vec_u8 output_data Null
        i64 ms_elapsed
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone completed_at Null
    }
    reading_sessions {
        i32 id PK
        i32 page Null
        f64 percentage_completed Null
        ReadiumLocator locator Null
        String epubcfi Null
        String koreader_progress Null
        DateTimeWithTimeZone started_at
        DateTimeWithTimeZone updated_at Null
        Uuid media_id FKey
        Uuid user_id FKey
        Uuid device_id FKNull
        i64 elapsed_seconds Null
    }
    emailer_send_records {
        i32 id PK
        i32 emailer_id FKey
        String recipient_email
        Vec_u8 attachment_meta Null
        DateTimeWithTimeZone sent_at
        UuidNull sent_by_user_id FK
    }
    user_preferences {
        i32 id PK
        InterfaceLayout preferred_layout_mode
        String locale
        String app_theme
        SupportedFont app_font
        String primary_navigation_mode
        i32 layout_max_width_px Null
        bool show_query_indicator
        bool enable_live_refetch
        bool enable_discord_presence
        bool enable_compact_display
        bool enable_gradients
        bool enable_double_sidebar
        bool enable_replace_primary_sidebar
        bool enable_hide_scrollbar
        bool enable_fancy_animations
        bool prefer_accent_color
        bool show_thumbnails_in_headers
        f32 thumbnail_ratio
        ThumbnailPlaceholderStyle thumbnail_placeholder_style
        bool enable_job_overlay
        bool enable_alphabet_select
        Arrangement navigation_arrangement Null
        Arrangement home_arrangement Null
        Uuid user_id Null
    }
    book_club_books {
        Uuid id PK
        i32 position
        DateTimeWithTimeZone completed_at Null
        String title Null
        String author Null
        String url Null
        String image_url Null
        Uuid book_entity_id FKNull
        Uuid book_club_id FKey
        DateTimeWithTimeZone added_at NotNull
    }
    registered_email_devices {
        i32 id PK
        String name
        String email
        bool forbidden
    }
    libraries {
        Uuid id PK
        String name
        String description Null
        String path
        FileStatus status
        ImageMetadata thumbnail_meta Null
        String thumbnail_path Null
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone updated_at Null
        String emoji Null
        i32 config_id FKey
        DateTimeWithTimeZoneNull last_scanned_at
    }
    favorite_libraries {
        Uuid user_id PKFK
        Uuid library_id PKFK
        DateTimeWithTimeZone favorited_at
    }
    reading_list_items {
        i32 id PK
        i32 display_order
        Uuid media_id FKey
        Uuid reading_list_id FKey
    }
    favorite_media {
        Uuid user_id PKFK
        Uuid media_id PKFK
        DateTimeWithTimeZone favorited_at
    }
    logs {
        i32 id PK
        LogLevel level
        String message
        DateTimeWithTimeZone timestamp
        Uuid job_id FKNull
        String context Null
    }
    server_invitations {
        String id PK
        String secret
        String email Null
        String granted_permissions Null
        DateTimeWithTimeZone created_at
        String expires_at
    }
    tags {
        i32 id PK
        String name
    }
    library_tags {
        i32 id PK
        Uuid library_id FKey
        i32 tag_id FKey
    }
    reviews {
        String id PK
        i32 rating
        StringNull content
        bool is_private
        String media_id FKey
        String user_id FKey
    }
    book_club_discussions {
        Uuid id PK
        bool is_locked
        bool is_archived
        Uuid book_club_book_id FKNull
        String title Null
        String emoji Null
        bool is_pinned
        DateTimeWithTimeZone created_at
        Uuid book_club_id FKNull
    }
    scheduled_job_libraries {
        i32 id PK
        i32 schedule_id FKey
        Uuid library_id FKey
    }
    media_annotations {
        Uuid id PK
        ReadiumLocator locator
        String annotation_text Null
        Uuid media_id FKey
        Uuid user_id FKey
        DateTimeUtc created_at
        DateTimeUtc updated_at NotNull
    }
    media_analysis {
        i32 id PK
        MediaAnalysisData data
        Uuid media_id FK
    }
    emailers {
        i32 id PK
        String name
        bool is_primary
        String sender_email
        String sender_display_name
        String username
        String encrypted_password
        String smtp_host
        i32 smtp_port
        bool tls_enabled
        i32 max_attachment_size_bytes Null
        i32 max_num_attachments Null
        DateTimeWithTimeZoneN last_used_at
    }
    book_club_discussion_message_reactions {
        Uuid id PK
        String emoji Null
        i32 custom_emoji_id FKNull
        DateTimeWithTimeZone created_at
        Uuid member_id FKey
        Uuid message_id FKey
    }
    reading_lists {
        Uuid id PK
        String name
        String description Null
        DateTimeWithTimeZone updated_at
        String visibility
        String ordering
        Uuid creating_user_id FKey
    }
    collections {
        String id PK
        String name
        String description Null
        DateTimeWithTimeZone updated_at NotNull
    }

    library_scan_records {
        i32 id PK
        Vec_u8 options Null
        DateTimeWithTimeZone timestamp
        Uuid library_id FKey
        UuidNull job_id FK
    }
    sessions {
        i32 id PK
        String session_id
        Uuid user_id FKey
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone expiry_time NotNull
    }
    age_restrictions {
        i32 id PK
        i32 age
        bool restrict_on_unset
        Uuid user_id FK
    }
    book_club_book_suggestions {
        Uuid id PK
        Uuid book_club_id FKey
        String title Null
        String author Null
        String url Null
        String notes Null
        BookClubSuggestionStatus status
        DateTimeWithTimeZone resolved_at Null
        Uuid resolved_by_id FKNull
        DateTimeWithTimeZone created_at
        Uuid suggested_by_id FKey
        UuidNull book_id FK
    }
    media_metadata {
        i32 id PK
        Uuid media_id FKNull
        i32 age_rating Null
        String characters Null
        String colorists Null
        String cover_artists Null
        String format Null
        i32 day Null
        String editors Null
        String genres Null
        String identifier_amazon Null
        String identifier_calibre Null
        String identifier_google Null
        String identifier_isbn Null
        String identifier_mobi_asin Null
        String identifier_uuid Null
        String inkers Null
        String language Null
        String letterers Null
        String links Null
        i32 month Null
        String notes Null
        f64 number Null
        i32 page_count Null
        String pencillers Null
        String publisher Null
        String series Null
        String series_group Null
        String story_arc Null
        f64 story_arc_number Null
        String summary Null
        String teams Null
        String title Null
        String title_sort Null
        i32 volume Null
        String writers Null
        i32 year Null
    }
    scheduled_job_configs {
        i32 id PK
        i32 interval_secs
    }
    last_library_visits {
        i32 id PK
        Uuid user_id FKey
        Uuid library_id FKey
        DateTimeWithTimeZone timestamp
    }
    library_configs {
        i32 id PK
        bool convert_rar_to_zip
        bool hard_delete_conversions
        ReadingDirection default_reading_dir
        ReadingMode default_reading_mode
        ReadingImageScaleFit default_reading_image_scale_fit
        bool generate_file_hashes
        bool generate_koreader_hashes
        bool process_metadata
        bool watch
        LibraryPattern library_pattern
        LibraryViewMode default_library_view_mode
        bool hide_series_view
        bool skip_book_overview
        ImageProcessorOptions thumbnail_config Null
        bool process_thumbnail_colors_even_without_config
        IgnoreRules ignore_rules Null
        UuidNull library_id
    }
    series {
        Uuid id PK
        String name
        String description Null
        DateTimeWithTimeZone created_at
        DateTimeWithTimeZone updated_at Null
        DateTimeWithTimeZone deleted_at Null
        String path
        FileStatus status
        ImageMetadata thumbnail_meta Null
        String thumbnail_path Null
        Uuid library_id FKNull
    }
    series_tags {
        i32 id PK
        Uuid series_id FKey
        i32 tag_id FKey
    }
    book_club_members {
        Uuid id PK
        String display_name Null
        String bio Null
        bool hide_progress
        BookClubMemberRole role
        DateTimeWithTimeZone joined_at
        Uuid user_id FKey
        Uuid book_club_id FKey
    }
    users ||--o{ favorite_series : "user_id -> id"
    series ||--o{ favorite_series : "series_id -> id"
    media ||--o{ media_tags : "media_id -> id"
    tags ||--o{ media_tags : "tag_id -> id"
    smart_lists ||--o{ smart_list_access_rules : "smart_list_id -> id"
    users ||--o{ smart_list_access_rules : "user_id -> id"
    libraries ||--o{ library_exclusions : "library_id -> id"
    users ||--o{ library_exclusions : "user_id -> id"
    media ||--o{ bookmarks : "media_id -> id"
    users ||--o{ bookmarks : "user_id -> id"
    book_clubs ||--o{ book_club_invitations : "book_club_id -> id"
    users ||--o{ book_club_invitations : "user_id -> id"
    series ||--o{ series_metadata : "series_id -> id"
    media ||--o{ finished_reading_sessions : "media_id -> id"
    registered_reading_devices |o--o{ finished_reading_sessions : "device_id -> id"
    users ||--o{ finished_reading_sessions : "user_id -> id"
    user_preferences |o--o{ users : "user_preferences_id -> id"
    reading_lists ||--o{ reading_list_rules : "reading_list_id -> id"
    book_club_members ||--o{ book_club_member_favorite_books : "member_id -> id"
    media |o--o{ book_club_member_favorite_books : "book_id -> id"
    book_club_book_suggestions ||--o{ book_club_book_suggestion_likes : "suggestion_id -> id"
    book_club_members ||--o{ book_club_book_suggestion_likes : "liked_by_id -> id"
    users ||--o{ refresh_tokens : "user_id -> id"
    users ||--o{ smart_lists : "creator_id -> id"
    smart_lists ||--o{ smart_list_views : "list_id -> id"
    users ||--o{ custom_emojis : "created_by_id -> id"
    book_clubs ||--o{ book_club_discussion_message : "book_club_id -> id"
    book_club_discussions ||--o{ book_club_discussion_message : "discussion_id -> id"
    book_club_members |o--o{ book_club_discussion_message : "member_id -> id"
    series |o--o{ media : "series_id -> id"
    users ||--o{ user_login_activity : "user_id -> id"
    users ||--o{ api_keys : "user_id -> id"
    media ||--o{ reading_sessions : "media_id -> id"
    registered_reading_devices |o--o{ reading_sessions : "device_id -> id"
    users ||--o{ reading_sessions : "user_id -> id"
    emailers ||--o{ emailer_send_records : "emailer_id -> id"
    users |o--o{ emailer_send_records : "sent_by_user_id -> id"
    book_clubs ||--o{ book_club_books : "book_club_id -> id"
    media |o--o{ book_club_books : "book_entity_id -> id"
    library_configs ||--o{ libraries : "config_id -> id"
    users ||--o{ favorite_libraries : "user_id -> id"
    libraries ||--o{ favorite_libraries : "library_id -> id"
    media ||--o{ reading_list_items : "media_id -> id"
    reading_lists ||--o{ reading_list_items : "reading_list_id -> id"
    users ||--o{ favorite_media : "user_id -> id"
    media ||--o{ favorite_media : "media_id -> id"
    jobs |o--o{ logs : "job_id -> id"
    libraries ||--o{ library_tags : "library_id -> id"
    tags ||--o{ library_tags : "tag_id -> id"
    media ||--o{ reviews : "media_id -> id"
    users ||--o{ reviews : "user_id -> id"
    book_clubs |o--o{ book_club_discussions : "book_club_id -> id"
    book_club_books |o--o{ book_club_discussions : "book_club_book_id -> id"
    libraries ||--o{ scheduled_job_libraries : "library_id -> id"
    scheduled_job_configs ||--o{ scheduled_job_libraries : "schedule_id -> id"
    media ||--o{ media_annotations : "media_id -> id"
    users ||--o{ media_annotations : "user_id -> id"
    media ||--o{ media_analysis : "media_id -> id"
    book_club_discussion_message ||--o{ book_club_discussion_message_reactions : "message_id -> id"
    book_club_members ||--o{ book_club_discussion_message_reactions : "member_id -> id"
    custom_emojis |o--o{ book_club_discussion_message_reactions : "custom_emoji_id -> id"
    users ||--o{ reading_lists : "creating_user_id -> id"
    jobs |o--o{ library_scan_records : "job_id -> id"
    libraries ||--o{ library_scan_records : "library_id -> id"
    users ||--o{ sessions : "user_id -> id"
    users ||--o{ age_restrictions : "user_id -> id"
    book_clubs ||--o{ book_club_book_suggestions : "book_club_id -> id"
    book_club_members ||--o{ book_club_book_suggestions : "suggested_by_id -> id"
    book_club_members |o--o{ book_club_book_suggestions : "resolved_by_id -> id"
    media |o--o{ book_club_book_suggestions : "book_id -> id"
    media |o--o{ media_metadata : "media_id -> id"
    libraries ||--o{ last_library_visits : "library_id -> id"
    users ||--o{ last_library_visits : "user_id -> id"
    libraries |o--o{ series : "library_id -> id"
    series ||--o{ series_tags : "series_id -> id"
    tags ||--o{ series_tags : "tag_id -> id"
    book_clubs ||--o{ book_club_members : "book_club_id -> id"
    users ||--o{ book_club_members : "user_id -> id"
```
