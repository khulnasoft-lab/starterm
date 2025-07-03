TARGET = starterm

ASSETS_DIR = extra
RELEASE_DIR = target/release
MANPAGE = $(ASSETS_DIR)/man/starterm.1.scd
MANPAGE-MSG = $(ASSETS_DIR)/man/starterm-msg.1.scd
MANPAGE-CONFIG = $(ASSETS_DIR)/man/starterm.5.scd
MANPAGE-CONFIG-BINDINGS = $(ASSETS_DIR)/man/starterm-bindings.5.scd
TERMINFO = $(ASSETS_DIR)/starterm.info
COMPLETIONS_DIR = $(ASSETS_DIR)/completions
COMPLETIONS = $(COMPLETIONS_DIR)/_starterm \
	$(COMPLETIONS_DIR)/starterm.bash \
	$(COMPLETIONS_DIR)/starterm.fish

# Benchmark variables
CARGO = cargo
BENCH_BIN_NAME = starbench
BENCH_SCRIPT = benchmark/clients/bench.sh
BENCH_DIR = /tmp/benchdir
BENCH_RESULTS_DIR = benchmark/results

# Termbench variables
TERMBENCH_DIR = termbench
TERMBENCH_BIN = termbench
TERMBENCH_RESULTS_DIR = $(TERMBENCH_DIR)/results
TERMBENCH_BENCHMARK_DIR = $(TERMBENCH_DIR)/benchmarks
TERMBENCH_GNUPLOT_DIR = $(TERMBENCH_DIR)/gnuplot

APP_NAME = Starterm.app
APP_TEMPLATE = $(ASSETS_DIR)/osx/$(APP_NAME)
APP_DIR = $(RELEASE_DIR)/osx
APP_BINARY = $(RELEASE_DIR)/$(TARGET)
APP_BINARY_DIR = $(APP_DIR)/$(APP_NAME)/Contents/MacOS
APP_EXTRAS_DIR = $(APP_DIR)/$(APP_NAME)/Contents/Resources
APP_COMPLETIONS_DIR = $(APP_EXTRAS_DIR)/completions

DMG_NAME = Starterm.dmg
DMG_DIR = $(RELEASE_DIR)/osx

vpath $(TARGET) $(RELEASE_DIR)
vpath $(APP_NAME) $(APP_DIR)
vpath $(DMG_NAME) $(APP_DIR)

all: help

help: ## Print this help message
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

binary: $(TARGET)-native ## Build a release binary
binary-universal: $(TARGET)-universal ## Build a universal release binary
$(TARGET)-native:
	MACOSX_DEPLOYMENT_TARGET="10.11" cargo build --release
$(TARGET)-universal:
	MACOSX_DEPLOYMENT_TARGET="10.11" cargo build --release --target=x86_64-apple-darwin
	MACOSX_DEPLOYMENT_TARGET="10.11" cargo build --release --target=aarch64-apple-darwin
	@lipo target/{x86_64,aarch64}-apple-darwin/release/$(TARGET) -create -output $(APP_BINARY)

app: $(APP_NAME)-native ## Create an Starterm.app
app-universal: $(APP_NAME)-universal ## Create a universal Starterm.app
$(APP_NAME)-%: $(TARGET)-%
	@mkdir -p $(APP_BINARY_DIR)
	@mkdir -p $(APP_EXTRAS_DIR)
	@mkdir -p $(APP_COMPLETIONS_DIR)
	@scdoc < $(MANPAGE) | gzip -c > $(APP_EXTRAS_DIR)/starterm.1.gz
	@scdoc < $(MANPAGE-MSG) | gzip -c > $(APP_EXTRAS_DIR)/starterm-msg.1.gz
	@scdoc < $(MANPAGE-CONFIG) | gzip -c > $(APP_EXTRAS_DIR)/starterm.5.gz
	@scdoc < $(MANPAGE-CONFIG-BINDINGS) | gzip -c > $(APP_EXTRAS_DIR)/starterm-bindings.5.gz
	@tic -xe starterm,starterm-direct -o $(APP_EXTRAS_DIR) $(TERMINFO)
	@cp -fRp $(APP_TEMPLATE) $(APP_DIR)
	@cp -fp $(APP_BINARY) $(APP_BINARY_DIR)
	@cp -fp $(COMPLETIONS) $(APP_COMPLETIONS_DIR)
	@touch -r "$(APP_BINARY)" "$(APP_DIR)/$(APP_NAME)"
	@codesign --remove-signature "$(APP_DIR)/$(APP_NAME)"
	@codesign --force --deep --sign - "$(APP_DIR)/$(APP_NAME)"
	@echo "Created '$(APP_NAME)' in '$(APP_DIR)'"

dmg: $(DMG_NAME)-native ## Create an Starterm.dmg
dmg-universal: $(DMG_NAME)-universal ## Create a universal Starterm.dmg
$(DMG_NAME)-%: $(APP_NAME)-%
	@echo "Packing disk image..."
	@ln -sf /Applications $(DMG_DIR)/Applications
	@hdiutil create $(DMG_DIR)/$(DMG_NAME) \
		-volname "Starterm" \
		-fs HFS+ \
		-srcfolder $(APP_DIR) \
		-ov -format UDZO
	@echo "Packed '$(APP_NAME)' in '$(APP_DIR)'"

install: $(INSTALL)-native ## Mount disk image
install-universal: $(INSTALL)-native ## Mount universal disk image
$(INSTALL)-%: $(DMG_NAME)-%
	@open $(DMG_DIR)/$(DMG_NAME)

.PHONY: app binary clean dmg install $(TARGET) $(TARGET)-universal

clean: ## Remove all build artifacts
	@cargo clean
	@rm -rf $(BENCH_DIR)

clean-bench-results: ## Clean benchmark results
	@rm -rf $(BENCH_RESULTS_DIR)

# Benchmark targets
bench-build: ## Build the benchmark project
	@cd benchmark && $(CARGO) build --release

bench-run: ## Run the benchmark server
	@cd benchmark && $(CARGO) run --release

bench: ## Run benchmarking script
	@echo "Running benchmark..."
	@if [ ! -d "$(BENCH_RESULTS_DIR)" ]; then mkdir -p $(BENCH_RESULTS_DIR); fi
	@cd benchmark && $(BENCH_SCRIPT) $(TARGET_DIR)/release/$(BENCH_BIN_NAME) ./

bench-test: ## Run benchmark tests
	@cd benchmark && $(CARGO) test

bench-fmt: ## Format benchmark code with rustfmt
	@cd benchmark && $(CARGO) fmt

bench-lint: ## Run clippy linter on benchmark code
	@cd benchmark && $(CARGO) clippy

bench-setup-db: ## Set up the benchmark database with diesel
	@cd benchmark && $(CARGO) install diesel_cli --no-default-features --features sqlite
	@cd benchmark && diesel setup
	@cd benchmark && diesel migration run

# Termbench targets
termbench-build: ## Build the termbench project
	@cd $(TERMBENCH_DIR) && $(CARGO) build --release

termbench-clean: ## Clean termbench build artifacts
	@cd $(TERMBENCH_DIR) && $(CARGO) clean

termbench-results-dir: ## Create results directory for termbench
	@mkdir -p $(TERMBENCH_RESULTS_DIR)

termbench-run: termbench-build termbench-results-dir ## Run terminal emulator benchmarks
	@cd $(TERMBENCH_DIR) && ./target/release/$(TERMBENCH_BIN) --dat "$(TERMBENCH_RESULTS_DIR)/$(shell date +%Y-%m-%d-%H%M%S).dat" $(TERMBENCH_BENCHMARK_DIR)

termbench-run-all: termbench-build termbench-results-dir ## Run all benchmarks including extra benchmarks
	@cd $(TERMBENCH_DIR) && ./target/release/$(TERMBENCH_BIN) --dat "$(TERMBENCH_RESULTS_DIR)/$(shell date +%Y-%m-%d-%H%M%S).dat" $(TERMBENCH_BENCHMARK_DIR) $(TERMBENCH_DIR)/extra_benchmarks

termbench-summary: ## Generate summary SVG from benchmark data files
	@cd $(TERMBENCH_RESULTS_DIR) && $(TERMBENCH_GNUPLOT_DIR)/summary.sh *.dat summary-$(shell date +%Y-%m-%d-%H%M%S).svg

termbench-detailed: ## Generate detailed SVG from benchmark data files
	@cd $(TERMBENCH_RESULTS_DIR) && $(TERMBENCH_GNUPLOT_DIR)/detailed.sh *.dat detailed-$(shell date +%Y-%m-%d-%H%M%S).svg

termbench-fmt: ## Format termbench code with rustfmt
	@cd $(TERMBENCH_DIR) && $(CARGO) fmt

termbench-lint: ## Run clippy linter on termbench code
	@cd $(TERMBENCH_DIR) && $(CARGO) clippy

termbench-test: ## Run termbench tests
	@cd $(TERMBENCH_DIR) && $(CARGO) test
