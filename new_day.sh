touch input/day$1.txt
touch input/day$1_test.txt
cp src/base.rs src/day$1.rs
dayinput=$1
echo $1
firstCharacter=${dayinput:0:1}
echo $firstCharacter
if [ $firstCharacter = "0" ];
then 
  dayaoc=${dayinput:1:2}
  echo $dayaoc
else
  dayaoc=$dayinput
fi
echo """
[[bin]]

name = "\"day$1\""
path = "\"src/day$1.rs\""
""" >> Cargo.toml

curl --cookie "session=53616c7465645f5f355049309925c7c9fdb4904500c2fd97e6be2841c2550710092fdd96e228b72cdd268e3914ede540" https://adventofcode.com/2021/day/$dayaoc/input > input/day$1.txt



