
# Carteira De Investimentos

Este projeto é baseado nas aulas ministradas no curso da Dio "Santander 2026 - Rust AI Developer".

O Objetivo desse  programa é possibilitar para o usuário administrador cadastrar diferentes moedas para invetimentos e cada usuário do sistema poder cadastrar as compras dessas moedas, definindo a data de compra, valor e quantidade.

Inicialmente as migrations já irá fazer o cadastro de algumas moedas e também suas cotações, baseadas no dia da geração das migrations.

Ao acessar o projeto o usuário se irá acessar a tela de login. Nessa tela terá o botão para registro de novos usuários, de forma simples. Para cadastrar um usuário com acesso de administrador, use o user name "admin" que internamente já está programado para definir o usuário com esse user name como administrador do sistema.

Segue também um .env de exemplo com os dados usados para conexão com o banco local durante o desenvolvimento desse desafio.

Foi utilizado no front end Tailwind para estilização e alpine.js para facilitar a abertura/fechamento de telas modais e accordions.

Enfim, o projeto está com um nível básico de validações, como por exemplo, a rota /moedas no front end só é acessada por usuário administrador.

Algumas rotas de API estão no projeto somente por terem sido feitas acompanhando as aulas finais, mas sem uso no projeto.

Obrigado pela oportunidade e pelas aulas ministradas.
