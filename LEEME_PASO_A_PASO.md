# Concat con traducción al español (parche)

Este parche añade al diálogo **Captions** de Concat un selector **Idioma: Original / Español / Ambos**.
Los subtítulos se generan con Whisper (en tu PC) y se traducen con Google Translate (gratis).

## Pasos (sin programar)
1. Entra a https://github.com/jub0t/Concat y pulsa **Fork → Create fork** (necesitas tu cuenta de GitHub).
2. En TU copia (fork), ve a **Add file → Upload files**.
3. Descomprime este ZIP y arrastra **todo su contenido** (la carpeta `.github` y la carpeta `src`) a la ventana. Pulsa **Commit changes**.
4. Pestaña **Actions**: acepta "I understand my workflows, go ahead and enable them". En la lista de la izquierda, **desactiva** los flujos CI, Mobile, Nix y Release (clic en cada uno → botón "..." → Disable workflow), para no gastar tu cuota gratis.
5. Elige **Build Windows (Concat con traduccion)** → **Run workflow** → Run. Tarda entre 30 y 60 minutos.
6. Al terminar con check verde, entra a la ejecución, baja a **Artifacts** y descarga **Concat-windows-x86_64**. Dentro hay un instalador `Concat-...-setup.exe`.
7. Si sale en rojo, copia las últimas líneas del error del paso que falló y envíamelas.

## Uso
Importa tu video a Concat, ponlo en la línea de tiempo, **selecciónalo**, abre **Captions**, elige modelo, **Idioma** y pulsa **Generate captions**. (La primera vez descarga un modelo de Whisper desde Ajustes › Transcriber.)
