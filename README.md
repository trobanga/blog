Letztes Update: 07.03.22

# Präambel

Hallo Leute,

ich möchte hier ein kleines Rustblog starten.
Ich programmiere seit etwa 2019 größtenteils in Rust und würde mich selbst als intermediären Rustprogrammierer bezeichnen, der in einigen Bereichen vielleicht schon an der Schwelle zum Spezialexperten steht.
Ziel dieses Blogs soll sein mein Wissen zu vertiefen und meine Projekte zu dokumentieren. 
Dabei werde ich versuchen möglichst viele Schritte und Gedankengänge zu dokumentieren und den Entwicklungsprozess nachvollziehbar zu machen.
Und vielleicht ist es für andere Programmier:innen auch nützlich.

Ich habe mich entschieden, diesen Blog auf deutsch zu schreiben, da es zum einen meine Muttersprache ist und zum anderen laut Rust Survey 2021 etwa 12% der Rustnutzer:innen deutsch sind und sich viele mehr Lernmaterial auf deutsch wünschen.
Da ich mich normalerweise größtenteils auf Englisch im Netz bewege und ausschließlich auf Englisch programmiere, möchte ich mich bereits im Vorfeld für das kommende Denglisch entschuldigen.

## Den Blog bootstrappen
Das erste Projekt soll dieser Blog werden, d.h. ich werde dokumentieren wie ich nach und nach den Blog baue und ihr könnt live dabei zusehen wie er sich ändert :)
Die Grundidee ist den Blog komplett in Rust zu betreiben und regelmäßig mit Artikeln aus Markdown zu füttern.

### Anforderungen
- Backend in Rust
  - dazu ein passendes Framework finden
- rein statischen Seiten
- Quellcode highlighting
- Projektthreads, ein Projekt ist eine Serie von Blogeinträgen
- auf uberspace hosten?
- Namen und Domain finden
- Markdown -> HTML Konverter

# Erste Schritte
- [x] Das Blogprojekt auf [Github](https://github.com/trobanga/blog) der Liste der verwaisten und vergessenen Projekte hinzufügen.
- [x] Einen passenden Markdown -> HTML Konverter finden
- [x] Auf [uberspace](https://uberspace.de) anmelden
- [x] Erste Seite veröffentlichen
- [ ] Namen und Domain finden
- [ ] _Werbung_ auf Discord, Reddit, und so schalten^^

## Markdown -> HTML Konverter
Eine kurze Internetsuche schlägt [markdown2html-converter](https://crates.io/crates/markdown2html-converter) vor, was sich mittels `cargo install markdown2html-converter` installieren lässt.
Mit dem Befehl `markdown2html-converter README.md` wird eine README.html erzeugt, die erstmal auch direkt gut aussieht.
Ich möchte allerdings mein eigenes CSS verwenden und das lässt das Crate nicht so ohne weiteres zu.
Eine Möglichkeit wäre einen Pullrequest zu machen, der die Funktionalität hinzufügt oder wir schauen kurz nach was in dem Crate eigentlich so passiert...

Intern verwendet `markdown2html-converter` ein Crate names [comrak](https://crates.io/crates/comrak), um die eigentliche Konvertierung durchzuführen.
Probieren wir das doch einfach aus: 

``` sh
cd blog
cargo new .
cargo add comrak
```

Hier ist mein minimaler Code, der die README.md nach HTML übersetzt.
```rust
use std::{fs::File, io::Read, io::Write};

use comrak::{format_html, parse_document, Arena, ComrakOptions};

fn main() {
    let mut f = File::open("README.md").unwrap();
    let mut md = String::new();
    f.read_to_string(&mut md).unwrap();

    let arena = Arena::new();
    let root = parse_document(&arena, &md, &ComrakOptions::default());
    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    let mut out = File::create("README.html").unwrap();

    out.write_all(b"<html>\n<body>\n").unwrap();
    out.write_all(&html).unwrap();
    out.write_all(b"</body>\n<html>\n").unwrap();
}
```

Ok, es hängt gerade etwas an der Namensfindung, deshalb mache ich erstmal mit der Blogstruktur weiter.

## Web-Framework
Blogs gibt es ja schon seit einer ganzen Weile und dementsprechend gibt es bereits diverse Frameworks dafür. 
[Zola](https://www.getzola.org/) ist ein Static Site Generator, der im Prinzip genau das macht was ich gerne hätte, aber der bringt mir einerseits zuviel Komplexität und gibt mir andererseits zu wenig Kontrolle über das was ich mache.
Außerdem wäre das ein bisschen zu einfach^^

Ich schwanke gerade zwischen den zwei Möglichkeiten ein einfaches Templatesystem zu schreiben oder schon etwas fertiges zu nehmen.
[Perseus](https://arctic-hen7.github.io/perseus/en-US/) gefällt mir ganz gut, ich bin mir nur noch nicht ganz sicher, ob das auch statische Seiten generieren kann.
In deren README steht, dass sie das unterstützen, aber ich habe noch nicht herausgefunden wie.

