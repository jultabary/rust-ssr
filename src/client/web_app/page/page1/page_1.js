class Product extends HTMLElement {
    constructor() {
        super();
        this.name = this.getAttribute('name');
        this.details = this.getAttribute('details')
        this.render();
    }

    render() {
        this.innerHTML = `<article><h2>${this.name}</h2><p>${this.details}</p></article>`
    }
}

async function get_all_products(product_path, list_id) {
    try {
        const réponse = await fetch(product_path,  { method: 'GET', headers: { 'Content-Type': 'application/json' }});
        if (réponse.status !== 200) {
            throw new Error('Erreur');
        }
        const products = await réponse.json();
        const list = getById(list_id);
        let innerHTML = '';
        for (const product of products) {
            innerHTML += `<li><product-component name="${product.name}" details="${product.details}" /></li>`
        }
        list.innerHTML = innerHTML;
    } catch(e) {
        console.error('Error when getting all products', e);
        getById('error-message').textContent = 'Error when getting all products';
    }
}
window.customElements.define('product-component', Product);
