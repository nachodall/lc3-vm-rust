; comment

.orig x3000

	lea R0, hello
	puts
	halt

hello: .stringz "Hello World!\n"
.end
