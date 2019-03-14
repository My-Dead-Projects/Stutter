
S -> expr

expr -> func | num | str | keyword | ident

func -> (expr expr {expr})

num -> digit {digit} [. digit {digit}]

str -> "{char}"

keyword -> define

ident -> {char}

digit -> 0-9

char -> a-z | A-Z
