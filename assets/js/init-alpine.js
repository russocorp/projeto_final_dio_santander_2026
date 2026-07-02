// /assets/js/init-alpine.js
function data() {
    return {
        isSideMenuOpen: false,
        isNotificationsMenuOpen: false,
        isProfileMenuOpen: false,
        modalAberto: false,
        moedaSelecionada: { id: null, nome: '', simbolo: '', valor: '' },
        moedaTransacaoSelecionada: { id: null, nome: '', valor_compra: 0, data: '', quantidade: 0 },
        modoModal: 'alterar',
        abrirModal(id, nome, simbolo, valor) {
            this.modoModal = 'alterar';
            this.moedaSelecionada = { id, nome, simbolo, valor };
            this.modalAberto = true;
        },
        abrirModalIncluir() {
            this.modoModal = 'incluir';
            this.moedaSelecionada = { id: null, nome: '', simbolo: '', valor: '' };
            this.modalAberto = true;
        },
        abrirModalTransacao(id, nome, valor_compra) {           
            const d = new Date();
            const ano = d.getFullYear();
            const mes = String(d.getMonth() + 1).padStart(2, '0');
            const dia = String(d.getDate()).padStart(2, '0');
            const hoje = `${ano}-${mes}-${dia}`;
            
            this.moedaTransacaoSelecionada = { id, nome, valor_compra, data: hoje, quantidade: 1 };
            this.modalAberto = true;
        },
        toggleSideMenu() {
            this.isSideMenuOpen = !this.isSideMenuOpen;
        },
        closeSideMenu() {
            this.isSideMenuOpen = false;
        },

        toggleNotificationsMenu() {
            this.isNotificationsMenuOpen = !this.isNotificationsMenuOpen;
        },
        closeNotificationsMenu() {
            this.isNotificationsMenuOpen = false;
        },

        toggleProfileMenu() {
            this.isProfileMenuOpen = !this.isProfileMenuOpen;
        },
        closeProfileMenu() {
            this.isProfileMenuOpen = false;
        }
    };
}