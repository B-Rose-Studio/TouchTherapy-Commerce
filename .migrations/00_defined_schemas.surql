// --- USERS
DEFINE TABLE OVERWRITE users TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON users TYPE string;

DEFINE FIELD OVERWRITE name ON users TYPE string;
DEFINE FIELD OVERWRITE email ON users TYPE string;
DEFINE FIELD OVERWRITE password ON users TYPE string;
DEFINE FIELD OVERWRITE cpf ON users TYPE string;
DEFINE FIELD OVERWRITE phone ON users TYPE string;
DEFINE FIELD OVERWRITE birth ON users TYPE datetime;

DEFINE FIELD OVERWRITE addresses ON users TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE addresses[*] ON users TYPE record<addresses>;
DEFINE FIELD OVERWRITE role ON users TYPE record<roles>;

DEFINE FIELD OVERWRITE verified ON users TYPE bool;
DEFINE FIELD OVERWRITE otp ON users TYPE bool;
DEFINE FIELD OVERWRITE secret_key ON users TYPE option<String>;

DEFINE FIELD OVERWRITE active ON users TYPE bool;
DEFINE FIELD OVERWRITE log ON users TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON users TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON users TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- ADDRESSES
DEFINE TABLE OVERWRITE addresses TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON addresses TYPE string;

DEFINE FIELD OVERWRITE name ON addresses TYPE string;
DEFINE FIELD OVERWRITE phone ON addresses TYPE string;
DEFINE FIELD OVERWRITE postal_code ON addresses TYPE string;
DEFINE FIELD OVERWRITE street_address ON addresses TYPE string;
DEFINE FIELD OVERWRITE number ON addresses TYPE string;
DEFINE FIELD OVERWRITE district ON addresses TYPE string;
DEFINE FIELD OVERWRITE state ON addresses TYPE string;
DEFINE FIELD OVERWRITE complement ON addresses TYPE string;

DEFINE FIELD OVERWRITE log ON addresses TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON addresses TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON addresses TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- PRODUCTS
DEFINE TABLE OVERWRITE products TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON products TYPE string;
DEFINE FIELD OVERWRITE name ON products TYPE string;
DEFINE FIELD OVERWRITE description ON products TYPE string;
DEFINE FIELD OVERWRITE price ON products TYPE float;
DEFINE FIELD OVERWRITE quantity ON products TYPE number;
DEFINE FIELD OVERWRITE sku ON products TYPE string;
DEFINE FIELD OVERWRITE images ON products TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE images[*] ON products TYPE string;

DEFINE FIELD OVERWRITE categories ON products TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE categories[*] ON products TYPE record<categories>;
DEFINE FIELD OVERWRITE supplier ON products TYPE record<suppliers>;

DEFINE FIELD OVERWRITE active ON products TYPE bool;
DEFINE FIELD OVERWRITE log ON products TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON products TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON products TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- SERVICES
DEFINE TABLE OVERWRITE services TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON services TYPE string;
DEFINE FIELD OVERWRITE name ON services TYPE string;
DEFINE FIELD OVERWRITE description ON services TYPE string;
DEFINE FIELD OVERWRITE images ON services TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE images[*] ON services TYPE string;
DEFINE FIELD OVERWRITE duration ON services TYPE number;
DEFINE FIELD OVERWRITE price ON services TYPE float;
DEFINE FIELD OVERWRITE sessions ON services TYPE number;

DEFINE FIELD OVERWRITE equipments ON services TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE equipments[*] ON services TYPE record<equipments>;

DEFINE FIELD OVERWRITE professionals ON services TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE professionals[*] ON services TYPE record<users>;

DEFINE FIELD OVERWRITE categories ON services TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE categories[*] ON services TYPE record<categories>;


DEFINE FIELD OVERWRITE active ON services TYPE bool;
DEFINE FIELD OVERWRITE log ON services TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON services TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON services TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- EQUIPMENTS
DEFINE TABLE OVERWRITE equipments TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON equipments TYPE string;
DEFINE FIELD OVERWRITE name ON equipments TYPE string;
DEFINE FIELD OVERWRITE serial ON equipments TYPE string;
DEFINE FIELD OVERWRITE status ON equipments TYPE "Operational" | "Broken" | "Missing" | "Maintenance";
DEFINE FIELD OVERWRITE quantity ON equipments TYPE number;
DEFINE FIELD OVERWRITE purchased_at ON equipments TYPE datetime VALUE type::datetime($value);
DEFINE FIELD OVERWRITE log ON equipments TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON equipments TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON equipments TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- SUPPLIERS
DEFINE TABLE OVERWRITE suppliers TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON suppliers TYPE string;
DEFINE FIELD OVERWRITE name ON suppliers TYPE string;
DEFINE FIELD OVERWRITE cnpj ON suppliers TYPE string;
DEFINE FIELD OVERWRITE email ON suppliers TYPE option<string>;
DEFINE FIELD OVERWRITE phone ON suppliers TYPE option<string>;

DEFINE FIELD OVERWRITE log ON suppliers TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON suppliers TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON suppliers TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- APPOINTMENTS
DEFINE TABLE OVERWRITE appointments TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON appointments TYPE string;

DEFINE FIELD OVERWRITE client ON appointments TYPE record<users>;
DEFINE FIELD OVERWRITE professional ON appointments TYPE record<users>;
DEFINE FIELD OVERWRITE service ON appointments TYPE record<services>;

DEFINE FIELD OVERWRITE sessions ON appointments TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE sessions[*] ON appointments TYPE {
    date_start:  option<datetime>,
    date_end:  option<datetime>,
    status: "Pending" | "Scheduled" | "Canceled" | "Completed" | "NoShow"
};

DEFINE FIELD OVERWRITE payment_method ON appointments TYPE "Pix" | "CreditcCard" | "DebitCard" | "Boleto";
DEFINE FIELD OVERWRITE payment_status ON appointments TYPE "Pending" | "Paid" | "Canceled" | "Reversed";

DEFINE FIELD OVERWRITE nfe ON appointments TYPE string;
DEFINE FIELD OVERWRITE observations ON appointments TYPE string;

DEFINE FIELD OVERWRITE log ON appointments TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON appointments TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON appointments TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- ORDERS
DEFINE TABLE OVERWRITE orders TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON orders TYPE string;
DEFINE FIELD OVERWRITE client ON orders TYPE record<users>;
DEFINE FIELD OVERWRITE address ON orders TYPE record<addresses>;

DEFINE FIELD OVERWRITE products ON orders TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE products[*] ON orders TYPE object;
DEFINE FIELD OVERWRITE products[*].product ON orders TYPE record<products>;
DEFINE FIELD OVERWRITE products[*].quantity ON orders TYPE number;
DEFINE FIELD OVERWRITE products[*].price ON orders TYPE float;

DEFINE FIELD OVERWRITE status ON orders TYPE "Preparing" | "Sent" | "Delivered" | "Reversed";
DEFINE FIELD OVERWRITE payment_method ON orders TYPE "Pix" | "CreditcCard" | "DebitCard" | "Boleto";
DEFINE FIELD OVERWRITE payment_status ON orders TYPE "Pending" | "Paid" | "Canceled" | "Reversed";

DEFINE FIELD OVERWRITE total ON orders TYPE number;
DEFINE FIELD OVERWRITE nfe ON orders TYPE string;
DEFINE FIELD OVERWRITE log ON orders TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON orders TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON orders TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- CARTS
DEFINE TABLE OVERWRITE carts TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON carts TYPE string;
DEFINE FIELD OVERWRITE user ON carts TYPE option<record<users>>;

DEFINE FIELD OVERWRITE products ON carts TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE products[*] ON carts TYPE record<products>;

DEFINE FIELD OVERWRITE services ON carts TYPE array DEFAULT [];
DEFINE FIELD OVERWRITE services[*] ON carts TYPE record<services>;

DEFINE FIELD OVERWRITE log ON carts TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON carts TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON carts TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- CATEGORIES
DEFINE TABLE OVERWRITE categories TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON categories TYPE string;
DEFINE FIELD OVERWRITE name ON categories TYPE string;
DEFINE FIELD OVERWRITE log ON categories TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON categories TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON categories TYPE datetime VALUE time::now();

// --------------------------------------------------------


// --- ROLES
DEFINE TABLE OVERWRITE roles TYPE NORMAL SCHEMAFULL;

DEFINE FIELD OVERWRITE id ON roles TYPE string;
DEFINE FIELD OVERWRITE name ON roles TYPE string;

DEFINE FIELD OVERWRITE users_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE users_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE products_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE products_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE categories_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE categories_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE services_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE services_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE appointments_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE appointments_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE equipments_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE equipments_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE suppliers_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE suppliers_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE orders_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE orders_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE carts_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE carts_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE roles_permissions ON roles TYPE array DEFAULT [] VALUE array::distinct($value);
DEFINE FIELD OVERWRITE roles_permissions[*] ON roles TYPE "Read" | "Create" | "Modify" | "Delete";

DEFINE FIELD OVERWRITE log ON roles TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE log.created_at ON roles TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE log.updated_at ON roles TYPE datetime VALUE time::now();

// --------------------------------------------------------
