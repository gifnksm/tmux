#include "tmux.h"

/* rust-glue.c */
int glue_window_panes_foreach_safe(struct window_panes *, void *,
                                   int (*)(struct window_pane *,
                                           void *context));
