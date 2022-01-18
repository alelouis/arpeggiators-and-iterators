Working directory for upcoming blog article about arpeggiators built from iterators.
**Still WIP**

---

### Running examples and converting to SVG
```bash
#!/bin/bash

# Convert a tinynotation string from rust binaries to .svg sheet music

# run rust bin and pass stdout as arg to lily converter
cargo run --bin $1 | xargs python tiny_to_ly.py $1  
# render to pdf with lilypond    
lilypond -dbackend=pdf -o ../out/ ../out/$1      
# convert to svg       
pdf2svg ../out/$1.pdf ../out/$1.svg    
# remove temp files                 
cd ../out && shopt -s extglob                           
rm !(*.svg)                                        
```
---
### From 
```rust
up.cycle()
    .take(16)
    .fold(String::new(), |acc, note| {
        let octave_str = (0..note.octave - 3).map(|_| "'").collect::<String>();
        acc + &format!("{:?}{octave_str}4", &note.letter).to_lowercase() + " "
    })                              
```

### To 
<p align="center">
  <img width="1000" src="out/up.svg">
</p>

