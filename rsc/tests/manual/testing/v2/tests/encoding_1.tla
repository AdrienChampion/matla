[test]
debug_only = false
release_only = false
expected = help me

---- MODULE encoding_1 ----
VARIABLES cnt, reset, start_stop, counting

LOCAL INSTANCE Integers
LOCAL sw_0 == INSTANCE sw_0
	WITH cnt <- cnt, reset <- reset, start_stop <- start_stop, counting <- counting

init == sw_0!init
next == sw_0!next

cnt_pos == cnt >= 0
====