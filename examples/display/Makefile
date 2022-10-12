#! /usr/bin/sh

# A makefile to handle building the following non-rust portions of the code

PACKAGE:=pivotable/examples/display
CSSDIR := ./src/css
TAILWIND := ./node_modules/tailwindcss
THEMES:=default


all: templates

clean:
	rm --force $(CSSDIR)/*.theme.css

templates: $(patsubst %,assets/%.theme.css,$(THEMES))

./assets/%.theme.css: $(TAILWIND) $(CSSDIR)/%.css
	mkdir -p assets \
	&& npx tailwindcss -i $(word 2,$^) -o $@

.PHONY: $(CSSDIR)/%.css
$(CSSDIR)/%.css:
	echo "There is no theme named $@ in the $(CSSDIR) directory" \
	&& exit 1

$(TAILWIND):
	npm install --save-dev tailwindcss

package.json:
	npm init --yes
