To test parsing all models from a folder:

``` sh
find . -iname '*tse' | tr '\n' '\0' | xargs -0 -n1 typhoondsl-outliner | grep -i failure | wc -l
```

``` sh
find . -iname '*tse' | tr '\n' '\0' | xargs -0 -n1 typhoondsl-outliner | grep -i success | wc -l
```
