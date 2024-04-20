async function add_product(url, callback, name_id, details_id, id_dialog) {
    const name = getById(name_id).value;
    const details = getById(details_id).value;
    const body = { name, details };
    const response = await fetch(url, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
    await response.json();
    getById(id_dialog).close();
    callback();
};
