DIST = dist
NODE_MODULES = node_modules

# delete a directory and report back, enforce deletion
define remove_dir
	@printf "$(1): " ; rm -r $(1) >/dev/null 2>&1 && echo "deleted." || echo "not found / not okay!"
endef

.PHONY: clean clean-bak clean-dist clean-node-modules clean-target server build docs test debug
clean-bak:
	@find . -name "*~" -exec rm -rfi {} \;

clean-dist:
	@$(call remove_dir,$(DIST))

clean-target:
	@cargo clean --quiet

clean-node-modules:
	@$(call remove_dir,$(NODE_MODULES))

clean: clean-bak clean-dist clean-node-modules clean-target

server:
	@trunk serve --open

build:
	@trunk build --release

debug:
	@trunk build

docs:
	@cargo doc --document-private-items --open

test:
	@cargo test
