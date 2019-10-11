#!/bin/sh

SED=$(command -v gsed || command -v sed)

top="$HOME/git/contactopensource"
src="$top/contactopensource_with_postgresql"
dst="$top/contactopensource_with_rust_diesel_postgresql"

sync(){
    rm "$dst/db/examples/"*
    rm -rf "$dst/db/migrations/"1*
    rsync -av "$src/db/" "$dst/db/"
    rsync -av "$src/doc/uml/" "$dst/doc/uml/"
}

migrate(){
    cd $dst && diesel migration run --migration-dir db/migrations
}

tune() {
    $SED -i 's/ Xml,/ Text,/; s/<Xml>/<Text>/;' src/schema.rs
}

sync
migrate
tune

