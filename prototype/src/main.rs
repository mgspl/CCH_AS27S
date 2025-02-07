#[derive(Clone)]
struct Cardapio<'a> {
    pub arroz: &'a str,
    pub feijao: &'a str,
    pub proteina: &'a str,
    pub guarnicao: &'a str,
    pub tamanho: &'a str,
}

fn main() {
    let marmita1 = Cardapio {
        arroz: "Branco",
        feijao: "Feijão",
        proteina: "Frango",
        guarnicao: "Mandioca Frita",
        tamanho: "Média",
    };

    // Clone via trait
    // Mudança de tamanho e proteina para marmita 2
    let mut marmita2 = marmita1.clone();
    marmita2.tamanho = "Pequena";
    marmita2.proteina = "Bife Acebolado";

    // Mudança de arroz, feijao e guarnicao para marmita 3
    let mut marmita3 = marmita1.clone();
    marmita3.arroz = "Temperado";
    marmita3.feijao = "Feijoada";
    marmita3.guarnicao = "Farofa";

    println!(
        "Marmita 1: \n Arroz: {} \n Feijão: {} \n Proteina: {} \n Guarnição: {} \n Tamanho: {} \n",
        marmita1.arroz, marmita1.feijao, marmita1.proteina, marmita1.guarnicao, marmita1.tamanho
    );
    println!(
        "Marmita 2: \n Arroz: {} \n Feijão: {} \n Proteina: {} \n Guarnição: {} \n Tamanho: {} \n",
        marmita2.arroz, marmita2.feijao, marmita2.proteina, marmita2.guarnicao, marmita2.tamanho
    );
    println!(
        "Marmita 3: \n Arroz: {} \n Feijão: {} \n Proteina: {} \n Guarnição: {} \n Tamanho: {} \n",
        marmita3.arroz, marmita3.feijao, marmita3.proteina, marmita3.guarnicao, marmita3.tamanho
    );
}
