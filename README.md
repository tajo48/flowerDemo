# Projekt na rekrutację do Hello IT

### Opis

Projekt ten został stworzony jako część procesu rekrutacyjnego do Hello IT. Jest to prosty program napisany w języku Rust, który tworzy wirtualnego kwiatka, który obraca się na ekranie i zmienia swój wygląd w czasie.

### Instrukcje

1. **Instalacja Rust:** 
   Zainstaluj język Rust z oficjalnej strony internetowej [rust-lang.org](https://www.rust-lang.org/learn/get-started).

2. **Uruchomienie programu:**
   Odpal program za pomocą komendy `cargo run`. Program powinien działać na systemach Linux, Windows i MacOS, ale został przetestowany tylko na systemie Linux i Windows (W windowsie terminal sypie errorami ale działa).

### Funkcjonalności

- **Obracający się kwiatek:** Po uruchomieniu programu, kwiatek będzie obracał się na ekranie.

- **Zmiana wyglądu:** Kwiatek zmienia swój wygląd w czasie na podstawie informacji przechowywanych w pliku `state.json`. Ten plik przechowuje czas narodzin kwiatka w formacie unix time oraz informacje o jego aktualnym wyglądzie.

- **Resetowanie stanu:** Aby zresetować stan rośnięcia kwiatka, należy usunąć plik `state.json`.

- **Dostosowywanie szybkości wzrostu:** Można dostosować czas potrzebny na rośnięcie kwiatka poprzez zmianę MAGICZNEGO NUMERKA w pliku `main.rs`. Im mniejsza wartość tego numerka, tym szybciej kwiatek będzie rósł, a im większa, tym wolniej.

### Informacja dodatkowa

Projekt wykorzystuje bibliotekę `bevy_psx`, która naśladuje wygląd grafiki generowanej przez procesor graficzny pierwszej konsoli PlayStation.
Obecnie jestem maintenerem tej biblioteki :3
