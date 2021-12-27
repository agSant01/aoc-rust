print() {
    echo -e $1
}

usage() {
    print "\nUsage:\n\t./get-input.sh <year> <day>"
}

if [[ -z "$1" ]]; then
    print "Error:\n\tMissing argument: <year>"
    usage
    exit 1;
fi

if [[ -z "$2" ]]; then
    print "Error:\n\tMissing argument: <day>"
    usage
    exit 1;
fi

if [[ -f ./$1/day$2/input/input.txt ]]; then
    print "Input file exits: ./$1/day$2/input/input.txt"
    exit 0;
fi

if [[ ! -d ./$1/day$2/ ]]; then
    print "ERROR:\n\tCargo project in \"./$1/day$2/\" does not exist."
    print "\t=> Create new cargo project first with \"cargo new <year>/<day>\". *day must be padded with '0'"
    exit 1;
fi

curl --header @./.env.headers https://adventofcode.com/2021/day/2/input > ./$1/day$2/input/input.txt
touch ./$1/day$2/input/input_test.txt