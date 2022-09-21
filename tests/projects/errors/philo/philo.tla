---- MODULE philo ----

EXTENDS Naturals

CONSTANT N

ASSUME N \IN Nat

init ==
    N = 0
step ==
    N' = N + 1

inv ==
    N > 0

====