08.04.2022

# Update - oder: ein Monat später 
Ok, so wie es aussieht, bin ich ein wenig vom Weg abgekommen.
Ich habe diesen [Talk](https://www.youtube.com/watch?v=9NDwHBjLlhQ) hier gesehen und habe mich davon inspirieren lassen...

Ob ich wirklich WAGI, WASI, und wie se nicht alle heißen verwende, weiß ich momentan noch nicht.
Aber [handlebars](https://crates.io/crates/handlebars) ist sehr interessant, da es das Templatesystem ist, welches auch für die offizielle Rust-Website und das Rustbuch verwendet wird.
Bleibt nur eine Frage: Brauche ich das überhaupt? 
Diese Frage kann man sich beim Progrmmieren nicht oft genug stellen und in den meisten Fällen, wie auch diesem, lautet die Antwort nein! 
Das heißt, vielleicht doch^^ wir werden sehen... 

Fassen wir kurz zusammen. 
Es gibt den Blog Gehirnrost auf gehirnrost.de (zumindest sobald ich das entsprechend eingerichtet habe).
Ich kann Markdown nach HTML konvertieren.
Das heißt der nächste Schritt ist ein HTML Grundgerüst zu bauen.
Ihr glaubt gar nicht wie wenig Lust ich dazu habe...aber gut. 
Oben ein Menü, darunter Content.

Das Menü soll die verschiedenen Artikelreihen enthalten, d.h. zuerst wird es nur einen Eintrag namens Blogbau geben und dann vielleicht noch eine Wilkommens-/Infoseite.
Insbesondere soll das Menü dabei automatisch durch den Content erzeugt werden.
Dazu werde ich einen Ordner content anlegen und jeder Unterordner steht für eine Artikelreihe.
In den Unterordnern liegen dann einzelne Markdowndateien, die die einzelnen Artikel enthalten.
Oder noch besser, jeder Artikel ist in einem weiteren Unterordner, das macht die Organisation einfacher, wenn noch Bilder etc. dazukommen.

``` sh
mkdir -p content/blogbau/{1,2}
```

Als erstes werden alle Projekte in content gelesen.
```rust
const CONTENT_DIR: &str = "./content";

fn main() -> io::Result<()> {
    for dir in std::fs::read_dir("./content")? {
        dbg!(dir?);
    }
    Ok(())
}
```
```stdout
[src/main.rs:13] dir? = DirEntry(
    "./content/blogbau",
)
```
