# Caminhos
$main = "src\main.rs"
$template = "src\ui\template.rs"

# Conteúdo original
$main_content = Get-Content $main -Raw
$template_content = Get-Content $template -Raw

# Inserção no main.rs após core_main().as_mut()
$main_modificado = $main_content -replace 'if let Some\(args\) = crate::core_main::core_main\(\)\.as_mut\(\) \{',
'if let Some(args) = crate::core_main::core_main().as_mut() {
    println!(">> Entrando em core_main().as_mut()");
    println!(">> Chamando ui::template::start()");'

# Inserções no template.rs
$template_modificado = $template_content `
    -replace 'let html = include_str!\("template/assets/welcome.html"\);',
'println!(">> Iniciando carregamento do HTML...");
    let html = include_str!("template/assets/welcome.html");' `
    -replace 'frame\.load_html\(html\.as_bytes\(\), Some\("welcome.html"\)\)',
'println!(">> Enviando HTML para frame...");
    frame.load_html(html.as_bytes(), Some("welcome.html"))' `
    -replace 'frame\.run_app\(\);',
'println!(">> Frame inicializado, entrando no loop da aplicação...");
    frame.run_app();'

# Salva os arquivos modificados
$template_modificado | Set-Content $template -Encoding UTF8
$main_modificado | Set-Content $main -Encoding UTF8

Write-Host "Modificações de debug inseridas com sucesso."
