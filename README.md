The following program defines a simple language, in which a program consists of assignments and each variable is assumed to be of the integer type. For the sake of simplicity, only operators that give integer values are included(Work on adding more types as I go).  This interpreter should be able to do the following for a given program: <br>(1) detect syntax errors; 
<br>(2) report uninitialized variables; <br> (3) perform the assignments if there is no error and print out the values of all the variables after all the assignments are done.<br>
<br>
___________________________
**Program:**<br>

     Assignment*

**Assignment:**<br>

    Identifier = Exp;

**Exp**<br>

    Exp + Term | Exp - Term | Term;

**Term**<br>

    Term * Fact  | Fact

**Fact**<br>

    ( Exp ) | - Fact | + Fact | Literal | Identifier

**Identifier**<br>

    Letter [Letter | Digit]*

**Letter**<br>

    a|...|z|A|...|Z|_

**Literal**<br>

    0 | NonZeroDigit Digit*

**NonZeroDigit**<br>

    1|...|9

**Digit**<br>

    0|1|...|9
______________________________


