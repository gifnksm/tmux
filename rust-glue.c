#include "tmux.h"
#include "rust-glue.h"

int glue_window_panes_foreach_safe(struct window_panes *panes, void *context, int (*f)(struct window_pane *, void *)) {
    struct window_pane *loopwp, *tmpwp;
    int ret = 0;
    TAILQ_FOREACH_SAFE(loopwp, panes, entry, tmpwp) {
        ret = f(loopwp, context);
        if (ret) {
            break;
        }
    }
    return ret;
}

int glue_winlinks_foreach_safe(struct winlinks *wls, void *context, int (*f)(struct winlink *, void *)) {
    struct winlink *loopwl, *tmpwl;
    int ret = 0;
    RB_FOREACH_SAFE(loopwl, winlinks, wls, tmpwl) {
        ret = f(loopwl, context);
        if (ret) {
            break;
        }
    }
    return ret;
}

struct winlink *glue_winlinks_prev(__unused struct winlinks *wls, struct winlink *wl) {
    return RB_PREV(winlinks, wls, wl);
}

struct winlink *glue_winlinks_next(__unused struct winlinks *wls, struct winlink *wl) {
    return RB_NEXT(winlinks, wls, wl);
}

void glue_window_remove_winlink(struct window *w, struct winlink *wl) {
    TAILQ_REMOVE(&w->winlinks, wl, wentry);
}

void glue_window_insert_winlink(struct window *w, struct winlink *wl) {
    TAILQ_INSERT_TAIL(&w->winlinks, wl, wentry);
}


struct session *glue_sessions_remove(struct session *s) {
    return RB_REMOVE(sessions, &sessions, s);
}

struct session *glue_sessions_insert(struct session *s) {
    return RB_INSERT(sessions, &sessions, s);
}


int glue_sessions_foreach_safe(struct sessions *ss, void *context, int (*f)(struct session *, void *)) {
    struct session *loops, *tmps;
    int ret = 0;
    RB_FOREACH_SAFE(loops, sessions, ss, tmps) {
        ret = f(loops, context);
        if (ret) {
            break;
        }
    }
    return ret;
}

int glue_clients_foreach_safe(struct clients *cs, void *ctx, int (*f)(struct client *, void *)) {
    struct client *loopc, *tmpc;
    int ret = 0;
    TAILQ_FOREACH_SAFE(loopc, cs, entry, tmpc) {
        ret = f(loopc, ctx);
        if (ret) {
            break;
        }
    }
    return ret;

}

struct window_pane *glue_window_first_pane(struct window *w) {
    return TAILQ_FIRST(&w->panes);
}

struct window_pane *glue_window_last_pane(struct window *w) {
    return TAILQ_LAST(&w->panes, window_panes);
}

void glue_window_replace_pane(struct window *w, struct window_pane *src,
                              struct window_pane *dst) {
    TAILQ_REPLACE(&w->panes, src, dst, entry);
}

void glue_window_insert_pane_head(struct window *w, struct window_pane *wp) {
    TAILQ_INSERT_HEAD(&w->panes, wp, entry);
}

void glue_window_insert_pane_after(struct window *w, struct window_pane *list_wp, struct window_pane *wp) {
    TAILQ_INSERT_AFTER(&w->panes, list_wp, wp, entry);
}


struct window_pane *glue_window_pane_prev(struct window_pane *wp) {
    return TAILQ_PREV(wp, window_panes, entry);
}
struct window_pane *glue_window_pane_next(struct window_pane *wp) {
    return TAILQ_NEXT(wp, entry);
}
