# You can test this in https://tio.run/#perl5

# Part 1
$_=<>;
s/(.).*/$&$1/;
s/(.)(?=\1)/$a+=$1/ge;
print$a;

# Part 2
$_=<>;
chomp;
$l=($L=y///c)/2-1;
$_ x=2;
s/(.)(?=.{$l}\1)/$a+=$1*($-[1]<$L)/ge;
print$a;