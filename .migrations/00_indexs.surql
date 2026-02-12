// --- INDEXES: USERS
DEFINE INDEX OVERWRITE userEmail ON users FIELDS email UNIQUE;
DEFINE INDEX OVERWRITE userCpf ON users FIELDS cpf UNIQUE;
DEFINE INDEX OVERWRITE userName ON users FIELDS full_name;

// --- INDEXES: PRODUCTS
DEFINE INDEX OVERWRITE productSku ON products FIELDS sku UNIQUE;
DEFINE INDEX OVERWRITE productName ON products FIELDS name;
DEFINE INDEX OVERWRITE productSupplier ON products FIELDS supplier;

// --- INDEXES: EQUIPMENTS
DEFINE INDEX OVERWRITE equipmentSerial ON equipments FIELDS serial UNIQUE;

// --- INDEXES: SUPPLIERS
DEFINE INDEX OVERWRITE supplierCnpj ON suppliers FIELDS cnpj UNIQUE;
DEFINE INDEX OVERWRITE supplierEmail ON suppliers FIELDS email UNIQUE;

// --- INDEXES: APPOINTMENTS
DEFINE INDEX OVERWRITE appointmentClient ON appointments FIELDS client;
DEFINE INDEX OVERWRITE appointmentProfessional ON appointments FIELDS professional;
DEFINE INDEX OVERWRITE appointmentStatus ON appointments FIELDS sessions[*].status;

// --- INDEXES: ORDERS
DEFINE INDEX OVERWRITE orderNfe ON orders FIELDS nfe UNIQUE;
DEFINE INDEX OVERWRITE orderClient ON orders FIELDS client;
DEFINE INDEX OVERWRITE orderStatus ON orders FIELDS status;

// --- INDEXES: CATEGORIES & ROLES
DEFINE INDEX OVERWRITE categoryName ON categories FIELDS name UNIQUE;
DEFINE INDEX OVERWRITE roleName ON roles FIELDS name UNIQUE;

// --- ANALYZER
DEFINE ANALYZER OVERWRITE search_analyzer TOKENIZERS blank,class,camel,punct FILTERS ascii,snowball(portuguese);

DEFINE INDEX OVERWRITE productNameSearch ON products FIELDS name SEARCH ANALYZER search_analyzer;
DEFINE INDEX OVERWRITE serviceNameSearch ON services FIELDS name SEARCH ANALYZER search_analyzer;
