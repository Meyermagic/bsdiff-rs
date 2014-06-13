

.PHONY : all
all: libbsdiff libbspatch


libbsdiff: bsdiff.rs deps/libbsdiff.a
	rustc --crate-type=rlib -L "./deps" -O bsdiff.rs

libbspatch: bspatch.rs deps/libbspatch.a
	rustc --crate-type=rlib -L "./deps" -O bspatch.rs


.PHONY : clean
clean:
	rm -f libbspatch*.rlib
	rm -f libbsdiff*.rlib