# Paths
SRC_STYLE = ./public/style
SRC_ASSETS = ./public/assets
SRC_PAGES = ./public/pages
TARGET_DIR = ./target
TARGET_CSS = $(TARGET_DIR)/style
TARGET_ASSETS = $(TARGET_DIR)/assets
TARGET_PAGES = $(TARGET_DIR)/pages

TAILWIND_CONF = ./tailwind.config.js
SRC_RUST = ./src/
env = dev

run: build
	@cargo run --color always

build: build_css copy_assets
	@cargo build --color always 
	@echo "Build Complete."


build_css:
	@rm -rf $(TARGET_CSS)
	@mkdir -p $(TARGET_CSS)
	@# v3.4
	@#tailwindcss -i $(SRC_STYLE)/main.css -c $(TAILWIND_CONF) -o $(TARGET_CSS)/style.css
	@# v4.0
	@npx tailwindcss -i $(SRC_STYLE)/style.css -o $(TARGET_CSS)/style.css
	@echo "CSS build complete."

copy_assets:
	@rm -rf $(TARGET_ASSETS)
	@mkdir -p $(TARGET_ASSETS)
	@mkdir -p $(TARGET_PAGES)
	@cp -r $(SRC_ASSETS)/* $(TARGET_ASSETS)
	@cp -r $(SRC_PAGES)/* $(TARGET_PAGES)
	@echo "Assets copied."

watch: 
	@watchexec -c -r -e scss,css,html,svg,rs make run


clean:
	# Clean the target directory
	@rm -rf $(TARGET_DIR)
	@echo "Cleaned build directory."
