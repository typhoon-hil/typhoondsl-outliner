# Testing the parser and getting informations

Go to the `examples` folder and do one of the commands bellow.

Number of models:

``` sh
find . -iname '*tse' | wc -l
```

Number of lines in all models:

``` sh
find . -iname '*tse' -print0 | xargs -0 cat | wc -l
```

Size in bytes of all models:

``` sh
find . -iname '*tse' -printf "%s\n" | awk '{sum+=$1} END{print sum}'
```

Number of failed models parsing:
``` sh
find . -iname '*tse' | tr '\n' '\0' | xargs -0 -n1 typhoondsl-outliner | grep -i failure | wc -l
```

Number of successful models parsing:
``` sh
find . -iname '*tse' | tr '\n' '\0' | xargs -0 -n1 typhoondsl-outliner | grep -i success | wc -l
```

