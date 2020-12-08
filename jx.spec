/* workflow */
%token LBRAC RBRAC LSQBRAC RSQBRAC LPAREN RPAREN COLON COMMA INTCONST DOUBLECONST BOOLCONST STRCONST NULL FOR IN IF ADD MINUS MUL DIV MOD NOT AND OR EQ NE LT LE GT GE ID
%start JX
%%

JX : LBRAC key_val_list RBRAC;

key_val_list : key_val | key_val COMMA key_val_list;
key_val : STRCONST COLON expr;
expr : value | jx_expr;
value : STRCONST
    | INTCONST
    | DOUBLECONST
    | BOOLCONST
    | object
    | list;

object : LBRAC RBRAC | LBRAC key_val_list RBRAC;

list : LSQBRAC RSQBRAC
    | LSQBRAC expr_list RSQBRAC
expr_list : expr | expr COMMA expr_list;

jx_expr : jx_arith_expr
    | list_compre_expr;

jx_arith_expr : arith_const ADD jx_arith_expr;
arith_const : STRCONST | INTCONST | DOUBLECONST;

list_compre_expr : value FOR ID IN iterable_expr opt_list_compre_expr;
iterable_expr : list;
opt_list_compre_expr : FOR ID IN iterable_expr opt_list_compre_expr;
