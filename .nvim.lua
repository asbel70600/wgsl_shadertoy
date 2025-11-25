-- Run Program
vim.cmd([[compiler cargo]])
vim.o.makeprg = [[cargo run]]
vim.o.errorformat = ""

local errorfm = table.concat({
	[[%-G%\s%#Some errors have detailed explanations%.%#]],
	[[%-G%\s%#For more information about an error, try%.%#]],
	[[%-G%\s%#Caused by:%.%#]],
	[[%-G%\s%#process didn't exit successfully%.%#]],
	[[%-G%\s%#Building%.%#]],
	[[%-G%\s%#Fresh%.%#]],
	[[%-G%.%#generated%.%#]],
	[[%-G]],
	[[%-Gerror: aborting %.%#]],
	[[%-Gerror: Could not compile %.%#]],
	[[%-G%.%#Finished %.%# profile%.%#]],
	[[%Eerror: %m]],
	[[%Eerror[E%n]: %m]],
	[[%Wwarning: %m]],
	[[%Inote: %m]],
	[[%C %#--> %f:%l:%c]],
	[[%E  left:%m]],
	[[%C right:%m %f:%l:%c]],
	[[%Z]],
	[[%-G%\s%#For more information about this error\]],
	[[%-Gnote: Run with `RUST_BACKTRACE=%.%#]],
	[[%.%#panicked at \'%m\'\]],
	[[%-A%.%#Running `target/%.%#]],
	[[%C%.%#]],
}, ",")
vim.o.errorformat = vim.o.errorformat .. "," .. errorfm

vim.keymap.set("n", "<Leader>r", function()
	vim.cmd([[make run]])
end)

-- %-G%\s%#Downloading%.%#
-- %-G%\s%#Checking%.%#
-- %-G%\s%#Compiling%.%#
-- %-G%\s%#Finished%.%#
-- %-G%\s%#error: Could not compile %.%#
-- %-G%\s%#To learn more\

-- [[%-G]],
-- [[%-Gerror: aborting %.%#]],
-- [[%-Gerror: Could not compile %.%#]],
-- [[%Eerror: %m]],
-- [[%Eerror[E%n]: %m]],
-- [[%Wwarning: %m]],
-- [[%Inote: %m]],
-- [[%C %#--> %f:%l:%c]],
-- [[%E  left:%m]],
-- [[%C right:%m %f:%l:%c]],
-- [[%Z]],
-- %f:%l:%c: %t%*[^:]: %m
-- %f:%l:%c: %*\d:%*\d %t%*[^:]: %m
-- %-G%f:%l %s
-- %-G%*[ ]^
-- %-G%*[ ]^%*[~]
-- %-G%*[ ]...
-- %.%#
-- %.%#
--  %f:%l:%c
--  try%.%#
-- %-G%\s%#Some errors have detailed explanations%.%#
-- %-G%\s%#For more information about an error
-- %-G%\s%#Caused by:%.%#
-- %-G%\s%#process didn't exit successfully%.%#
-- %-G%\s%#Building%.%#
-- %-G%\s%#Fresh%.%#
-- %-G%.%#generated%.%#
