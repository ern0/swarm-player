#!/bin/bash

function gen {

  pandoc \
    -t html \
    --self-contained \
    --css=style.css \
    $1.md \
    -o /tmp/$1.html

  wkhtmltopdf \
    /tmp/$1.html \
    $1.pdf

  rm -f /tmp/$1.html

}

gen design
