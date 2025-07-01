use sciter::Window;

pub fn start(_args: &mut [String]) {
    #[cfg(target_os = "windows")]
    {
        // Para compatibilidade futura (se necessário)
    }

    println!(">> Entrando em ui::template::start()");
    let mut frame = Window::new();
    println!(">> Iniciando carregamento do HTML...");

    let html = include_str!("template/assets/welcome.html");
    println!(">> Enviando HTML para frame...");
    if !frame.load_html(html.as_bytes(), Some("welcome.html")) {
        println!("!! Erro ao carregar o HTML.");
        return;
    }

    println!(">> Frame inicializado, entrando no loop da aplicação...");
    frame.run_app();
}

