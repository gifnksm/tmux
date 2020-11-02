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
