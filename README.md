# Touch Therapy

### Descrição
E-commerce de produtos de auto cuidado e sistema de gestao de agendamentos para clinica de estetica.

## 1. Entidades (Modelagem de Dados)

### **Usuários (`users`)**
Representa qualquer pessoa que interage com o sistema.
- `id`: (UUID/RecordID) Identificador único.
- `nome_completo`: (String) Nome e sobrenome.
- `email`: (String) Chave única para login/contato.
- `cpf`: (String) Identificação fiscal (único).
- `telefone`: (String) Para contato ou notificações (WhatsApp/SMS).
- `data_nascimento`: (DateTime) Para verificar idade/promoções.
- `cargo`: (Enum) `Cliente`, `Funcionario`, `Admin`.
- `ativo`: (Bool) Soft delete (para não perder histórico se o usuário sair).
- `criado_em`: (DateTime) Data de registro.

### **Autenticação (`auth_codes`)**
Gerencia o login sem senha (Passwordless).
- `id`: (UUID).
- `email_vinculado`: (String) Quem solicitou.
- `codigo`: (String/Hash) O código de 6 dígitos ou token mágico.
- `tipo`: (Enum) `Login`, `ConfirmacaoEmail`.
- `expira_em`: (DateTime) Validade curta (ex: 15 min).
- `usado`: (Bool) Se já foi consumido.

### **Fornecedores (`suppliers`)**
Origem dos produtos físicos.
- `id`: (UUID).
- `nome_empresa`: (String).
- `cnpj`: (String).
- `contato_email`: (String).
- `ativo`: (Bool).

### **Produtos (`products`)**
Itens físicos de auto cuidado.
- `id`: (UUID).
- `nome`: (String).
- `descricao`: (String) Texto rico ou HTML sanitizado.
- `preco`: (Decimal) Valor monetário.
- `estoque_qtd`: (Int) Quantidade física atual.
- `sku`: (String) Código de controle de estoque.
- `categoria`: (String) Ex: "Óleos", "Cremes", "Acessórios".
- `fornecedor_id`: (Link) Relação com tabela Fornecedores.
- `imagens`: (List<String>) URLs das fotos.
- `ativo`: (Bool).

### **Equipamentos (`equipments`)**
Máquinas necessárias para realizar tratamentos.
- `id`: (UUID).
- `nome`: (String) Ex: "Laser Galaxy X".
- `numero_serie`: (String).
- `status`: (Enum) `Operacional`, `Manutencao`, `Quebrado`.
- `ultima_manutencao`: (DateTime).
- `proxima_manutencao`: (DateTime).

### **Serviços (`services`)**
Tratamentos oferecidos pela clínica.
- `id`: (UUID).
- `nome`: (String) Ex: "Depilação a Laser".
- `descricao`: (String).
- `duracao_minutos`: (Int) Ex: 30, 60, 90.
- `preco`: (Decimal).
- `equipamento_id`: (Link/Optional) Se o serviço depende de uma máquina específica.
- `ativo`: (Bool).

### **Agendamentos (`appointments`)**
A reserva de horário na agenda.
- `id`: (UUID).
- `usuario_id`: (Link) Cliente.
- `servico_id`: (Link) Tratamento.
- `data_hora_inicio`: (DateTime).
- `data_hora_fim`: (DateTime) Calculado via `inicio` + `duracao_servico`.
- `status`: (Enum) `Pendente`, `Confirmado`, `Concluido`, `CanceladoCliente`, `CanceladoAdmin`.
- `observacoes`: (String) Notas do cliente ou do terapeuta.

### **Carrinho (`carts`)**
Estado temporário de compra.
- `id`: (UUID).
- `usuario_id`: (Link/Optional) Se logado.
- `session_token`: (String) Para usuários não logados.
- `itens`: (Array<Object>) Lista contendo ID do produto/serviço e quantidade.
- `atualizado_em`: (DateTime) Para limpeza de carrinhos abandonados.

### **Vendas/Pedidos (`orders`)**
Registro imutável de uma transação finalizada.
- `id`: (UUID).
- `usuario_id`: (Link).
- `status`: (Enum) `AguardandoPagamento`, `Pago`, `Enviado`, `Entregue`, `Estornado`.
- `total`: (Decimal).
- `metodo_pagamento`: (Enum) `Pix`, `CartaoCredito`, `Boleto`.
- `itens_snapshot`: (JSON) Cópia dos dados dos produtos no momento da compra (preço congela aqui).
- `data_criacao`: (DateTime).

---

## 2. RFs (Requisitos Funcionais)

### Módulo de Autenticação e Perfil
- [ ] O usuário deve poder se cadastrar (sign-up).
- [ ] O usuário deve poder solicitar acesso via "Magic Link" ou código OTP (login).
- [ ] O sistema deve enviar o código para o e-mail cadastrado.
- [ ] O usuário deve poder validar o código para receber o token de sessão (JWT).
- [ ] O usuário deve poder fazer logout (invalidar sessão local).
- [ ] O usuário deve poder editar seus dados pessoais (exceto CPF).

### Módulo Loja (Cliente)
- [ ] O usuário deve poder listar produtos com paginação.
- [ ] O usuário deve poder filtrar produtos por categoria e faixa de preço.
- [ ] O usuário deve poder buscar produtos por nome.
- [ ] O usuário deve poder ver detalhes de um produto (descrição, preço, fotos).
- [ ] O usuário deve poder adicionar/remover produtos do carrinho.
- [ ] O usuário deve poder alterar a quantidade de itens no carrinho.
- [ ] O usuário deve visualizar o resumo do carrinho (subtotal) a qualquer momento.

### Módulo Clínica (Cliente)
- [ ] O usuário deve poder visualizar a lista de serviços disponíveis.
- [ ] O sistema deve exibir visualmente se um serviço está "Indisponível" (caso o equipamento esteja quebrado).
- [ ] O usuário deve poder consultar horários livres em uma data específica.
- [ ] O usuário deve poder agendar um serviço (adicionando ao carrinho ou agendamento direto).
- [ ] O usuário deve poder visualizar seus agendamentos futuros ("Minha Agenda").
- [ ] O usuário deve poder cancelar um agendamento (sujeito às RNs).

### Módulo Checkout
- [ ] O usuário deve poder escolher o método de pagamento.
- [ ] O sistema deve validar o pagamento (mock ou integração real).
- [ ] O usuário deve receber um e-mail de confirmação de pedido/agendamento.
- [ ] O usuário deve ter acesso ao histórico de pedidos antigos.

### Módulo Administrativo (Backoffice)
- [ ] O administrador deve ter um Dashboard com: Faturamento do dia, Próximos agendamentos, Alertas de estoque baixo.
- [ ] **Gestão de Produtos:** CRUD completo de produtos e fornecedores.
- [ ] **Gestão de Estoque:** Ajuste manual de quantidade (entrada de nota fiscal).
- [ ] **Gestão de Serviços:** CRUD de tratamentos e preços.
- [ ] **Gestão de Equipamentos:** Cadastro e alteração de status (`Operacional` -> `Quebrado`).
- [ ] **Gestão de Agenda:** O admin pode bloquear horários manualmente (férias/feriados).
- [ ] **Gestão de Usuários:** Listar usuários e desativar contas suspeitas.
- [ ] **Gestão de Pedidos:** Alterar status do pedido (ex: de "Pago" para "Enviado").

---

## 3. RNFs (Requisitos Não-Funcionais)

- [ ] **Linguagem Backend:** 100% Rust (garantia de memória e performance).
- [ ] **Frontend:** Dioxus (Compilação para WebAssembly/WASM).
- [ ] **Banco de Dados:** SurrealDB (NoSQL/Graph multi-model).
- [ ] **Arquitetura:** Clean Architecture / Hexagonal (Domínio isolado de Frameworks).
- [ ] **Modularidade:** Uso de Cargo Workspace (`core`, `adapter`, `api`, `web`).
- [ ] **Autenticação:** Stateless via JWT (JSON Web Token) com assinatura segura.
- [ ] **Criptografia:** Dados sensíveis em repouso e HTTPS em trânsito.
- [ ] **Logs e Rastreabilidade:** Uso de `tracing` (crate) para logs estruturados.
- [ ] **Tratamento de Erros:** O sistema nunca deve "panicar" (crashar) por erro de usuário; deve retornar erros controlados (`Result<T, E>`).
- [ ] **Internacionalização de Datas:** O backend opera estritamente em UTC; o frontend converte para o fuso horário local do usuário.
- [ ] **Resiliência:** O sistema de envio de e-mails deve ser assíncrono (background task) para não travar o request do usuário.

---

## 4. RNs (Regras de Negócios)

### Cadastro e Acesso
- [ ] **Dados Obrigatórios:** Nome, Sobrenome, E-mail, CPF, Telefone e Data de Nascimento são mandatórios.
- [ ] **Unicidade:** Não podem existir dois usuários ativos com o mesmo CPF ou E-mail.
- [ ] **Maioridade:** (Opcional) Apenas maiores de 18 anos podem agendar certos tratamentos invasivos.
- [ ] **Senha:** O sistema não armazena senhas. O acesso é exclusivo via Token/OTP.

### Loja e Estoque
- [ ] **Reserva de Estoque:** O estoque só é efetivamente baixado na confirmação do pagamento.
- [ ] **Estoque Negativo:** O sistema deve impedir a venda se `quantidade_carrinho > estoque_atual`.
- [ ] **Limite de Compra:** Máximo de 10 unidades do mesmo produto por pessoa (para evitar revenda não autorizada).
- [ ] **Precificação:** O preço exibido deve ser o final. Impostos já calculados na inserção do produto.

### Agendamento e Equipamentos
- [ ] **Dependência Crítica:** Se o status de um Equipamento for alterado para "Quebrado" ou "Manutenção", todos os Serviços vinculados a ele ficam bloqueados para novos agendamentos imediatamente.
- [ ] **Conflito de Agenda:** O sistema deve garantir atomicidade: dois usuários tentando agendar o mesmo horário no mesmo milissegundo, apenas um consegue.
- [ ] **Cancelamento:** Cancelamentos feitos com menos de 24h de antecedência não geram reembolso automático (ou geram apenas 50%).
- [ ] **Reagendamento:** O usuário pode reagendar livremente se faltarem mais de 48h para a sessão.
- [ ] **Intervalo:** Deve haver um intervalo automático de 10 minutos entre sessões para higienização da sala (se aplicável).

---
## 📡 Endpoints da API (V1)

| Categoria | Método | Rota | Protegido |
| --- | --- | --- | --- |
| **Auth** | `POST` | `/api/auth/register` | ❌ |
| **Auth** | `POST` | `/api/auth/login` | ❌ |
| **Auth** | `POST` | `/api/auth/reset-password` | ❌ |
| **Auth** | `GET` | `/api/auth/user` | ✅ |
| --- | --- | --- | --- |

---

## GUIDE COMMANDS
