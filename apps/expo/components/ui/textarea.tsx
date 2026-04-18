import * as React from 'react'
import { TextInput, type TextInputProps } from 'react-native'

import { cn } from '~/lib/utils'

const Textarea = React.forwardRef<React.ElementRef<typeof TextInput>, TextInputProps>(
	({ className, multiline = true, numberOfLines = 4, placeholderClassName, ...props }, ref) => {
		return (
			<TextInput
				ref={ref}
				className={cn(
					'web:flex border-input native:text-lg native:leading-[1.25] web:ring-offset-background web:focus-visible:outline-none web:focus-visible:ring-2 web:focus-visible:ring-ring web:focus-visible:ring-offset-2 squircle rounded-md px-3 py-2 text-base lg:text-sm min-h-[80px] w-full border bg-background text-foreground placeholder:text-foreground-muted',
					props.editable === false && 'web:cursor-not-allowed opacity-50',
					className,
				)}
				placeholderClassName={cn('text-foreground-muted', placeholderClassName)}
				multiline={multiline}
				numberOfLines={numberOfLines}
				textAlignVertical="top"
				{...props}
			/>
		)
	},
)

Textarea.displayName = 'Textarea'

export { Textarea }
