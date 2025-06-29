import os
import time
import subprocess
from pathlib import Path

WATCHED_EXTENSIONS = ['.htm', '.html', '.js', '.css', '.tis', '.sciter']
WATCH_FOLDER = Path('src/ui')
SCRIPT_TO_RUN = Path('res/inline-sciter.py')

def get_file_mtimes(folder):
    mtimes = {}
    for path in folder.rglob("*"):
        if path.suffix.lower() in WATCHED_EXTENSIONS:
            mtimes[str(path)] = path.stat().st_mtime
    return mtimes

def run_inline_script():
    print("[INFO] Alterações detectadas. Executando inline-sciter.py ...")
    result = subprocess.run(['python', str(SCRIPT_TO_RUN)], capture_output=True, text=True)
    if result.returncode != 0:
        print("[ERRO] Falha ao executar inline-sciter.py")
        print(result.stderr)
    else:
        print("[OK] Arquivo ui_inline.rs regenerado com sucesso.")

def main():
    print("[MONITOR] Aguardando alterações em arquivos de UI ...")
    last_mtimes = get_file_mtimes(WATCH_FOLDER)
    while True:
        time.sleep(1)
        current_mtimes = get_file_mtimes(WATCH_FOLDER)
        if current_mtimes != last_mtimes:
            run_inline_script()
            last_mtimes = current_mtimes

if __name__ == "__main__":
    main()
