#include "tmux.h"

/* rust-glue.c */
int glue_window_panes_foreach_safe(struct window_panes *, void *,
                                   int (*)(struct window_pane *,
                                           void *context));
int glue_winlinks_foreach_safe(struct winlinks *, void *,
                               int (*)(struct winlink *, void *context));
struct winlink *glue_winlinks_prev(struct winlinks *, struct winlink *);
struct winlink *glue_winlinks_next(struct winlinks *, struct winlink *);
void glue_window_remove_winlink(struct window *, struct winlink *);
void glue_window_insert_winlink(struct window *, struct winlink *);
struct session *glue_sessions_remove(struct session *s);
struct session *glue_sessions_insert(struct session *s);
int glue_sessions_foreach_safe(struct sessions *, void *,
                               int (*f)(struct session *, void *));
int glue_clients_foreach_safe(struct clients *, void *,
                              int (*f)(struct client *, void *));
struct window_pane *glue_window_first_pane(struct window *);
struct window_pane *glue_window_last_pane(struct window *);
void glue_window_replace_pane(struct window *, struct window_pane *,
                              struct window_pane *);
void glue_window_insert_pane_head(struct window *, struct window_pane *);
void glue_window_insert_pane_after(struct window *, struct window_pane *,
                                   struct window_pane *);
struct window_pane *glue_window_pane_prev(struct window_pane *);
struct window_pane *glue_window_pane_next(struct window_pane *);
