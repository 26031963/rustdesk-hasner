
# Caminho para o arquivo index.tis
$arquivo = "C:\ProjetoRustDesk-Hasner\rustdesk\src\ui\index.tis"

# Ler o conteúdo do arquivo
$conteudo = Get-Content $arquivo

# Procurar linhas do bloco $(function() { ... });
$inicio = $null
$fim = $null
for ($i = 0; $i -lt $conteudo.Length; $i++) {
    if ($conteudo[$i] -match "^\$\(function\s*\(\)\s*\{") {
        $inicio = $i
    }
    if ($inicio -ne $null -and $conteudo[$i] -match "^\s*\}\);\s*$") {
        $fim = $i
        break
    }
}

if ($inicio -ne $null -and $fim -ne $null) {
    # Novo bloco válido
    $novo_bloco = @(
        '$(function() {',
        '    var ui = <div style="size:*">',
        '        <App />',
        '        <div #msgbox />',
        '    </div>;',
        '    $(body).content(ui);',
        '});'
    )

    # Substituir bloco no conteúdo original
    $antes = $conteudo[0..($inicio - 1)]
    $depois = $conteudo[($fim + 1)..($conteudo.Length - 1)]
    $novo_conteudo = $antes + $novo_bloco + $depois

    # Salvar de volta no arquivo
    $novo_conteudo | Set-Content $arquivo -Encoding UTF8
    Write-Output "Bloco corrigido com sucesso."
} else {
    Write-Output "Bloco `$(`function() { ... })` não encontrado corretamente para substituição."
}
