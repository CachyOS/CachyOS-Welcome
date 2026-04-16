# About dialog
about-dialog-title = CachyOS Hello
about-dialog-comments = Tela de Boas-Vindas do CachyOS

# Tweaks page
tweaks = Ajustes
fixes = Utilitários
applications = Aplicações
removed-db-lock = O arquivo de bloqueio do banco de dados (db.lock) do Pacman foi removido!
lock-doesnt-exist = O arquivo db.lock do Pacman não existe!
orphans-not-found = Não foram encontrados pacotes órfãos!
package-not-installed = O pacote '{$package_name}' não foi instalado!
gaming-package-installed = Os pacotes de jogos já estão instalados!
winboat-package-installed = Os pacotes do Winboat já estão instalados!
gpu-boosters-package-installed = Os pacotes de otimização de GPU já estão instalados!

# Application Browser page
advanced-btn = Avançado
reset-btn = Redefinir
update-system-app-btn = Atualizar Sistema
application-column = Aplicativo
description-column = Descrição
install-remove-column = Instalar / Remover
advanced-btn-tooltip = Exibe opções adicionais de aplicativos e pacotes
reset-btn-tooltip = Descarta suas mudanças atuais
update-system-app-btn-tooltip = Aplicar as mudanças ao sistema

# Dns Connections page
dns-settings = Configurações do DNS
select-connection = Selecionar Conexão:
select-dns-server = Selecionar servidor DNS:
apply = Aplicar
reset = Redefinir
enable-dot = Ativar DNS sobre TLS (DoT)
dot-tooltip = Criptografa as consultas DNS usando TLS para maior privacidade (requer suporte do servidor)
enable-doh = Ativar DNS sobre HTTPS (DoH)
doh-tooltip = Criptografa as consultas DNS usando HTTPS através de um proxy local blocky (requer suporte do servidor e instalar o blocky)
doh-blocky-install-failed = Falha ao instalar o blocky para suporte a DoH!
test-latency = Testar latência do servidor selecionado
test-latency-tooltip = Medir a latência de rede até o servidor DNS selecionado
best-server = Selecionar melhor servidor por latência
best-server-tooltip = Testar servidores DNS base (excluindo variantes de filtragem) e selecionar o mais rápido
latency-result = {""}
server-info = {""}
latency-testing = testando...
latency-timeout = tempo esgotado
latency-no-result = nenhum servidor respondeu
custom-dns = Personalizado
dhcp-automatic = DHCP (automático)
custom-dns-ipv4 = Endereços IPv4 (separados por vírgula):
custom-dns-ipv6 = Endereços IPv6 (separados por vírgula):
custom-dns-dot-hostname = Nome de host DoT (opcional):
custom-dns-invalid = Por favor, insira pelo menos um endereço IPv4 ou IPv6
custom-dns-invalid-hostname = Nome de host DoT inválido
custom-dns-doh-url = URL do DoH (para DNS sobre HTTPS):
custom-dns-doh-url-required = Por favor, insira uma URL DoH válida começando com https://
dns-check-hint = Após aplicar, verifique seu provedor DNS em
dns-server-changed = O servidor DNS foi trocado com sucesso!
dns-server-failed = Erro ao mudar o servidor DNS!
dns-server-reset = O servidor DNS foi redefinido!
dns-server-reset-failed = Erro ao redefinir o servidor DNS!
winboat-install-failed = Erro ao instalar o Winboat!

# Tweaks page (tweaks)
tweak-enabled-title = {$tweak} ativado
tweak-psd-tooltip = Usa a memória RAM para os perfis do navegador (mais rápido e com menor desgaste do disco)
tweak-oomd-tooltip = Encerra os processos automaticamente quando a memória está baixa para evitar travamentos
tweak-bpftune-tooltip = Ajusta automaticamente os parâmetros da rede do sistema
tweak-bluetooth-tooltip = Ativa o suporte para dispositivos Bluetooth (mouses, áudio, etc.)
tweak-ananicycpp-tooltip = Ajusta automaticamente as prioridades dos processos para melhorar a fluidez do sistema
tweak-cachyupdate-tooltip = Ativa o serviço de notificações de atualizações na bandeja do sistema

# Tweaks page (fixes)
remove-lock-title = Remover bloqueio do banco de dados (db.lock) do Pacman
reinstall-title = Reinstalar Todos os Pacotes
reset-keyrings-title = Redefinir chaves de assinatura
update-system-title = Atualizar Sistema
remove-orphans-title = Remover Pacotes Órfãos
clear-pkgcache-title = Limpar cache de pacotes
rankmirrors-title = Ranquear Espelhos
dnsserver-title = Trocar Servidor DNS
show-kwinw-debug-title = Exibir a Janela de Depuração do KWin(Wayland)
install-gaming-title = Instalar Pacotes de Jogos
install-winboat-title = Instalar Winboat
install-gpu-boosters-title = Instalar Otimizadores de GPU
install-gpu-boosters-tooltip = Instala os pacotes dmemcg-booster e plasma-foreground-booster em GPUs AMD ou Intel

# Main Page (buttons)
button-about-tooltip = Sobre
button-web-resource-tooltip = Página Web
button-development-label = Desenvolvimento
button-software-label = Aplicações do CachyOS
button-donate-label = Doar
button-forum-label = Fórum
button-installer-label = Instalar CachyOS
button-involved-label = Contribua no Projeto
button-readme-label = Leia-Me
button-release-info-label = Informações da Versão
button-wiki-label = Wiki

# Main Page (sections)
section-docs = DOCUMENTAÇÃO
section-installer = INSTALAÇÃO
section-support = SUPORTE
section-project = PROJETO

# Main Page (launch installer)
recommended = recomendado
calamares-install-type = Calamares install type

# Main Page (body)
offline-error = Não foi possível iniciar a instalação! Sem conexão com a internet.
unsupported-hw-warning = Você está tentando instalar o CachyOS em um hardware incompatível com a versão atual. Sua versão não é elegível para suporte.
desktop-on-handheld-error = Você está tentando instalar a edição Desktop em um dispositivo portátil. Por favor, use a edição Handheld para suporte adequado neste hardware
outdated-version-warning = Você está usando uma versão antiga do CachyOS, considere instalar uma versão atualizada.
testing-iso-warning = Você está usando uma ISO de testes que não é estável para uso.
tweaksbrowser-label = Aplicativos / Ajustes
appbrowser-label = Instalar Aplicativos
launch-start-label = Abrir ao Iniciar
welcome-title = Bem-Vindo ao CachyOS!
welcome-body =
    Obrigado por entrar na nossa comunidade!

    Nós, os desenvolvedores do CachyOS, esperamos que você goste de usá-lo o tanto quanto gostamos de desenvolvê-lo! Os links abaixo ajudarão você dar os primeiros passos com seu novo sistema operacional. Aproveite a experiência, e não hesite em dar seu feedback!
