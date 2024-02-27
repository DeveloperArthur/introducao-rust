fn main1() {
    //declarando vetor
    let v: Vec<i32> = Vec::new();

    //Isso criará um novo Vec <i32> que contém os valores 1, 2 e 3:
    let v = vec![1, 2, 3];

    //modificando vetor:
    let mut vm: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /*lembrando que o vetor e seus elementos serão
    descartados se sairem de escopo*/

    //duas formas de acessar valores do vetor:
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // esse daria ERRO se o indice nao existisse
    let third: Option<&i32> = v.get(2); // esse nao daria erro, retornaria None

    //referencias invalidas:
    let mut v = vec![1, 2, 3, 4, 5]; // <- vetor mutavel

    let first = &v[0]; // <- referencia imutavel

    v.push(6); // <- ERRO cannot borrow as mutable

    /*Este código pode parecer que deveria funcionar: por que uma 
    referência ao primeiro elemento deveria se preocupar com o que 
    muda sobre o final do vetor? A razão porque este código não é 
    permitido é devido à forma como os vetores funcionam. 
    Adicionando um novo elemento no final do vetor pode exigir a 
    atribuição de nova alocação de memória e copiar os elementos 
    antigos para o novo espaço, na circunstância de não haver espaço 
    suficiente para colocar todos os elementos próximos um do outro 
    onde o vetor estava. Nesse caso, a referência ao primeiro 
    elemento apontaria para memória não alocada. 
    As regras de empréstimo impedem que os programas acabem nessa 
    situação. */
}

//Usando um Enum para Armazenar Vários Tipos:

/*os vetores só podem armazenar valores que são todos 
do mesmo tipo, há casos de uso para a necessidade de 
armazenar uma lista de coisas de diferentes tipos.
quando precisamos armazenar elementos de um tipo diferente 
em um vetor, podemos definir e usar um enum*/

enum CelulaDaPlanilha {
    Int(i32),
    Text(String),
    Float(f64),
}

fn main2() {
    let mut linha_da_planilha: Vec<CelulaDaPlanilha> = Vec::new();
    linha_da_planilha.push(CelulaDaPlanilha::Int(3));
    linha_da_planilha.push(CelulaDaPlanilha::Text(String::from("blue")));
    linha_da_planilha.push(CelulaDaPlanilha::Float(10.23));
}

//porque não usar tuplas neste caso?

//RESPOSTA: porque não conseguimos adicionar valores em uma tupla
// uma vez que ela é declarada, é fixa.
fn main() {
    let linha_da_planilha: (i32, String, f64);

    linha_da_planilha = (3, String::from("blue"), 10.23)

    // nao consigo adicionar mais elementos para a linha da planilha
}

// com struct-tupla:
fn main() {
    struct CelulaDaPlanilha(i32, String, f64);

    let linha1 = CelulaDaPlanilha(3, String::from("blue"), 10.23);
    let linha2 = CelulaDaPlanilha(4, String::from("red"), 23.00);

    /*por mais que a gente consiga criar linhas, elas nao estao
    de fato relacionadas, é como se criassemos os elementos, mas
    eles nao estao em uma lista, elementos soltos, alem disso
    nao conseguimos adicionar valores na linha... pois
    struct-tuplas se comportam como tuplas, e tuplas sao fixas */
}