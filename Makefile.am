# Makefile.am

# Obvious program stuff.
bin_PROGRAMS = tmux
CLEANFILES = tmux.1.mdoc tmux.1.man cmd-parse.c

# Distribution tarball options.
EXTRA_DIST = \
	CHANGES README README.ja COPYING example_tmux.conf \
	osdep-*.c mdoc2man.awk tmux.1
dist_EXTRA_tmux_SOURCES = compat/*.[ch]

# Preprocessor flags.
AM_CPPFLAGS += @XOPEN_DEFINES@ \
	-DTMUX_VERSION='"@VERSION@"' \
	-DTMUX_CONF='"$(sysconfdir)/tmux.conf:~/.tmux.conf:$$XDG_CONFIG_HOME/tmux/tmux.conf:~/.config/tmux/tmux.conf"'

# Additional object files.
LDADD = $(LIBOBJS) $(RUST_LIB)

# Set flags for gcc.
if IS_GCC
AM_CFLAGS += -std=gnu99 -O2
if IS_DEBUG
AM_CFLAGS += -g
AM_CFLAGS += -Wno-long-long -Wall -W -Wformat=2
AM_CFLAGS += -Wmissing-prototypes -Wstrict-prototypes -Wmissing-declarations
AM_CFLAGS += -Wwrite-strings -Wshadow -Wpointer-arith -Wsign-compare
AM_CFLAGS += -Wundef -Wbad-function-cast -Winline -Wcast-align
AM_CFLAGS += -Wdeclaration-after-statement -Wno-pointer-sign -Wno-attributes
AM_CFLAGS += -Wno-unused-result
AM_CPPFLAGS += -DDEBUG
endif
AM_CPPFLAGS += -iquote.
endif

# Set flags for Solaris.
if IS_SUNOS
if IS_GCC
AM_CPPFLAGS += -D_XPG6
else
AM_CPPFLAGS += -D_XPG4_2
endif
endif

# Set flags for Sun CC.
if IS_SUNCC
AM_CFLAGS += -erroff=E_EMPTY_DECLARATION
endif

# Set _LINUX_SOURCE_COMPAT for AIX for malloc(0).
if IS_AIX
AM_CPPFLAGS += -D_LINUX_SOURCE_COMPAT=1
endif

# Set flags for NetBSD.
if IS_NETBSD
AM_CPPFLAGS += -D_OPENBSD_SOURCE
endif

# List of sources.
dist_tmux_SOURCES = \
	alerts.c \
	arguments.c \
	attributes.c \
	cfg.c \
	client.c \
	cmd-attach-session.c \
	cmd-break-pane.c \
	cmd-capture-pane.c \
	cmd-choose-tree.c \
	cmd-command-prompt.c \
	cmd-confirm-before.c \
	cmd-copy-mode.c \
	cmd-detach-client.c \
	cmd-display-menu.c \
	cmd-display-message.c \
	cmd-display-panes.c \
	cmd-find-window.c \
	cmd-find.c \
	cmd-if-shell.c \
	cmd-join-pane.c \
	cmd-list-keys.c \
	cmd-load-buffer.c \
	cmd-move-window.c \
	cmd-new-session.c \
	cmd-new-window.c \
	cmd-parse.y \
	cmd-paste-buffer.c \
	cmd-pipe-pane.c \
	cmd-queue.c \
	cmd-refresh-client.c \
	cmd-resize-pane.c \
	cmd-resize-window.c \
	cmd-respawn-pane.c \
	cmd-respawn-window.c \
	cmd-rotate-window.c \
	cmd-run-shell.c \
	cmd-save-buffer.c \
	cmd-select-layout.c \
	cmd-select-pane.c \
	cmd-select-window.c \
	cmd-send-keys.c \
	cmd-set-buffer.c \
	cmd-set-environment.c \
	cmd-set-option.c \
	cmd-show-environment.c \
	cmd-show-messages.c \
	cmd-show-options.c \
	cmd-source-file.c \
	cmd-split-window.c \
	cmd-switch-client.c \
	cmd-wait-for.c \
	cmd.c \
	compat.h \
	control-notify.c \
	control.c \
	environ.c \
	file.c \
	format.c \
	format-draw.c \
	grid-view.c \
	grid.c \
	input-keys.c \
	input.c \
	job.c \
	key-bindings.c \
	key-string.c \
	layout-custom.c \
	layout-set.c \
	layout.c \
	log.c \
	menu.c \
	mode-tree.c \
	names.c \
	notify.c \
	options-table.c \
	options.c \
	paste.c \
	popup.c \
	proc.c \
	regsub.c \
	resize.c \
	rust-glue.c \
	rust-glue.h \
	screen-redraw.c \
	screen-write.c \
	screen.c \
	server-client.c \
	server-fn.c \
	server.c \
	session.c \
	spawn.c \
	status.c \
	style.c \
	tmux.c \
	tmux.h \
	tty-acs.c \
	tty-features.c \
	tty-keys.c \
	tty-term.c \
	tty.c \
	utf8.c \
	window-buffer.c \
	window-client.c \
	window-clock.c \
	window-copy.c \
	window-customize.c \
	window-tree.c \
	window.c \
	xmalloc.c \
	xmalloc.h
nodist_tmux_SOURCES = osdep-@PLATFORM@.c

# Add compat file for forkpty.
if NEED_FORKPTY
nodist_tmux_SOURCES += compat/forkpty-@PLATFORM@.c
endif

# Add compat file for utf8proc.
if HAVE_UTF8PROC
nodist_tmux_SOURCES += compat/utf8proc.c
endif

# Install tmux.1 in the right format.
install-exec-hook:
	if test x@MANFORMAT@ = xmdoc; then \
		sed -e "s|@SYSCONFDIR@|$(sysconfdir)|g" $(srcdir)/tmux.1 \
			>$(srcdir)/tmux.1.mdoc; \
	else \
		sed -e "s|@SYSCONFDIR@|$(sysconfdir)|g" $(srcdir)/tmux.1| \
			$(AWK) -f $(srcdir)/mdoc2man.awk >$(srcdir)/tmux.1.man; \
	fi
	$(mkdir_p) $(DESTDIR)$(mandir)/man1
	$(INSTALL_DATA) $(srcdir)/tmux.1.@MANFORMAT@ \
		$(DESTDIR)$(mandir)/man1/tmux.1

RUST_SOURCES = \
	Cargo.toml \
	build.rs \
	tmux.h \
	rust-glue.h \
	src/arguments.rs \
	src/client.rs \
	src/cmd.rs \
	src/cmd/bind_key.rs \
	src/cmd/kill_pane.rs \
	src/cmd/kill_server.rs \
	src/cmd/kill_session.rs \
	src/cmd/kill_window.rs \
	src/cmd/list_buffers.rs \
	src/cmd/list_clients.rs \
	src/cmd/list_panes.rs \
	src/cmd/list_sessions.rs \
	src/cmd/list_windows.rs \
	src/cmd/lock_client.rs \
	src/cmd/lock_server.rs \
	src/cmd/lock_session.rs \
	src/cmd/rename_session.rs \
	src/cmd/rename_window.rs \
	src/cmd/start_server.rs \
	src/cmd/swap_pane.rs \
	src/cmd/swap_window.rs \
	src/cmd/unbind_key.rs \
	src/cmd/unlink_window.rs \
	src/cmd_find.rs \
	src/cmd_parse.rs \
	src/cmd_queue.rs \
	src/colour.rs \
	src/ffi.rs \
	src/format.rs \
	src/key_bindings.rs \
	src/key_string.rs \
	src/layout.rs \
	src/lib.rs \
	src/notify.rs \
	src/options.rs \
	src/paste.rs \
	src/resize.rs \
	src/server.rs \
	src/session.rs \
	src/window.rs

RUST_EXTRA = \
	Cargo.lock

EXTRA_DIST += $(RUST_SOURCES) $(RUST_EXTRA)

if IS_DEBUG
CARGO_RELEASE_ARGS=
else
CARGO_RELEASE_ARGS=--release
endif

CARGO_VERBOSE = $(cargo_verbose_$(V))
cargo_verbose_ = $(cargo_verbose_$(AM_DEFAULT_VERBOSITY))
cargo_verbose_0 =
cargo_verbose_1 = --verbose

RUST_LIB=@abs_top_builddir@/target/@RUST_TARGET_SUBDIR@/libtmux.a

$(RUST_LIB): $(RUST_SOURCES)
	CARGO_TARGET_DIR=@abs_top_builddir@/target cargo build @RUST_TARGET_FLAG@ $(CARGO_VERBOSE) $(CARGO_RELEASE_ARGS)

check-local:
	CARGO_TARGET_DIR=@abs_top_builddir@/target cargo test $(CARGO_VERBOSE)

clean-local:
	CARGO_TARGET_DIR=@abs_top_builddir@/target cargo clean
