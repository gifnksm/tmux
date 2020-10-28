#include "tmux.h"

/* rust-glue.c */
int glue_window_panes_foreach_safe(struct window_panes *, void *,
                                   int (*)(struct window_pane *,
                                           void *context));
int glue_winlinks_foreach_safe(struct winlinks *, void *,
                               int (*)(struct winlink *, void *context));
int glue_sessions_foreach_safe(struct sessions *, void *,
                               int (*f)(struct session *, void *));
