if exists("b:current_syntax")
  finish
endif

syn keyword c4Keyword use foreign fn struct enum sum union impl type const pub as
syn keyword c4Conditional if elif else match
syn keyword c4Repeat while loop
syn keyword c4Statement return break continue
syn keyword c4Operator and or not

syn keyword c4Type u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str bool void

syn keyword c4Intrinsic sizeof alignof
syn match c4PointerIntrinsic "@read\>"
syn match c4PointerIntrinsic "@write\>"
syn match c4PointerIntrinsic "@advance\>"
syn match c4PointerIntrinsic "@offset\>"
syn match c4PointerIntrinsic "@diff\>"

syn keyword c4Boolean true false
syn keyword c4Null null

syn match c4Float "\<\d\+\.\d\+\>"
syn match c4Hex "\<0x\x\+\>"
syn match c4Dec "\<\d\+\>"

syn match c4SpecialChar "\\\([ntr0\\'"abfv]\|x[0-9a-fA-F]\{2\}\)" contained
syn region c4String start='"' end='"' contains=c4SpecialChar
syn region c4Char start="'" end="'" contains=c4SpecialChar

syn region c4Comment start="//" end="$" contains=@Spell

syn match c4FuncName "\%(\<fn\s\+\)\@<=[a-zA-Z_][a-zA-Z0-9_]*"
syn match c4StructName "\%(\<\%(struct\|enum\|sum\|union\|type\|impl\)\s\+\)\@<=[a-zA-Z_][a-zA-Z0-9_]*"

syn match c4CustomType "\<[A-Z][a-zA-Z0-9_]*\>"

syn match c4Delimiter "::"

hi def link c4Keyword Keyword
hi def link c4Conditional Conditional
hi def link c4Repeat Repeat
hi def link c4Statement Statement
hi def link c4Operator Keyword
hi def link c4Type Type
hi def link c4Intrinsic Keyword
hi def link c4PointerIntrinsic Special
hi def link c4Boolean Boolean
hi def link c4Null Constant
hi def link c4Float Float
hi def link c4Hex Number
hi def link c4Dec Number
hi def link c4SpecialChar SpecialChar
hi def link c4String String
hi def link c4Char Character
hi def link c4Comment Comment
hi def link c4FuncName Function
hi def link c4StructName Typedef
hi def link c4CustomType Type
hi def link c4Delimiter Special

let b:current_syntax = "c4"
