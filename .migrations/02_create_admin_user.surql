INSERT INTO users {
    id: "d1aa397c-ce20-46f7-ac19-8923bec5ea85",

    name: "Admin System",
    email: "",
    password: "",
    cpf: "",
    phone: "",
    birth: time::now(),

    addresses: [],

    verified: true,
    otp: false,
    secret_key: None,

    role: (SELECT VALUE id FROM roles WHERE name = 'AdminRole')[0],
    active: true,
}
