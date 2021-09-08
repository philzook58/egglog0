  $ $TESTDIR/run_test.sh arith.pl
  Results : 
  -? (mul two two) = (plus two two)
  [];
  -? (mul two two) = (plus one ?X)
  [?X = three];
  -? (mul two two) = ?Z
  [?Z = four];
  -? (plus (mul x three) (mul two (plus one (mul four x)))) = ?Z
  [?Z = (plus two (mul x (plus five six)))];
  
