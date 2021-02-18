#define ENDARR	{ 0,	XK_Escape, 	"",	quit,	{ .i = 0 } }, \
		{ 0,	0,		NULL,	NULL,	0 }
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }

#include "keymacros.h"

static Key treeroot[] = {
	/*mask			keysym	string	function	argument*/
	{ KEY_a,		quit,		{ .i = 0 } },
	{ KEY_q,		tree,		{ .v = quitb } },
	ENDARR
};
