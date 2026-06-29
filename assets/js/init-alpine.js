// /assets/js/init-alpine.js
function data() {
    return {
        isSideMenuOpen: false,
        isNotificationsMenuOpen: false,
        isProfileMenuOpen: false,
        modalAberto: false,
        moedaSelecionada: { id: null, nome: '', simbolo: '', valor: '' },
        moedaTransacaoSelecionada: { id: null, nome: '', valor: '', data: '', quantidade: '' },
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
        abrirModalTransacao(id, nome, valor) {            
            this.moedaTransacaoSelecionada = { id, nome, valor, data: new Date(), quantidade: 1 };
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