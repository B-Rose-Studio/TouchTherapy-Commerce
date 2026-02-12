INSERT INTO roles {
    id: "cf674798-7f52-4feb-88a5-1c3c479a1413",
    name: "ClientRole",
    products_permissions: ['Read'],
    categories_permissions: ['Read'],
    services_permissions: ['Read'],
};

INSERT INTO roles {
    id: "93a333a1-829f-4636-b7a0-fc5ac5a75ee1",
    name: "AdminRole",

    users_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    products_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    categories_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    services_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    appointments_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    equipments_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    suppliers_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    orders_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    carts_permissions: ['Read', 'Create', 'Modify', 'Delete'],
    roles_permissions: ['Read', 'Create', 'Modify', 'Delete'],
}
