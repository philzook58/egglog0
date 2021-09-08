  $ $TESTDIR/run_test.sh cat1.pl
  Results : 
  -? p = q
  [];
  -? f = g
  unknown.
  -? p = f
  unknown.
  -? k = h
  unknown.
  -? k = g
  unknown.
  -? (type (comp p h)) = ?T
  [?T = (hom z c)];
  -? (type (id a)) = (hom a a)
  [];
  -? (comp (comp (id a) h) k) = ?T
  [?T = (comp f g)];
  
