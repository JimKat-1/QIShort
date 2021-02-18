#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <ctype.h>
#include <X11/Xlib.h>
#include <X11/keysym.h>

#define LENGTH(X)		(sizeof(X) / sizeof(X[0]))
#define ERROR(text)		{ perror(text); _exit(1); }

typedef union {
	int i;
	unsigned int ui;
	float f;
	const void *v;
} Arg;

typedef struct {
	int mask;
	int keysym;
	char *string;
	void (*func)(const Arg *);
	const Arg arg;
} Key;

static void grabkeyboard();
static void keyevent(XEvent *e);
static void loop();
static void spawn(const Arg *arg);
static void quit(const Arg *arg);
static void tree(const Arg *arg);

#include "config.h"

Display *dpy;
Window root;
Key *curarr = treeroot;
char text[256] = { 0 };

void
grabkeyboard(){
	dpy = XOpenDisplay(NULL);
	if(!dpy)
		ERROR("Could not open display");
	root = DefaultRootWindow(dpy);

	XSelectInput(dpy, root, KeyPressMask);

	if(XGrabKeyboard(dpy, root, True, GrabModeAsync, GrabModeAsync, CurrentTime) != GrabSuccess)
		_exit(1);
}

void
keypress(XKeyEvent *ev){
	KeySym keysym;
	keysym = XKeycodeToKeysym(dpy, (KeyCode)ev->keycode, 0);
	if (XK_Shift_L <= keysym && XK_Hyper_R >= keysym ||		// definitions at
			keysym == XK_Scroll_Lock ||                     // /usr/include/X11/keysymdef.h
			keysym == XK_Num_Lock) //Return if control key  //
		return;
	for (int i = 0; curarr[i].keysym; i++){
		if ((keysym == curarr[i].keysym) &&
		(ev->state & (ShiftMask|ControlMask|Mod1Mask)) == curarr[i].mask){
			strcat(text,curarr[i].string);
			printf("%s\n",text);
			curarr[i].func(&(curarr[i].arg));
			return;
		}
	}
	_exit(0);
}

void
loop(){
	XEvent xev;

	while(1){
		XNextEvent(dpy, &xev);
		if (xev.xkey.type == KeyPress)
			keypress(&xev.xkey);
	}
}

void
quit(const Arg *arg){
	_exit(arg->i);
}

void
spawn(const Arg *arg){
	if(!fork())
		execvp(((char**)arg->v)[0], (char**)arg->v);
	_exit(0);
}

void
tree(const Arg *arg){
	curarr = arg->v;
}

int
main(){
	grabkeyboard();
	loop();
}
