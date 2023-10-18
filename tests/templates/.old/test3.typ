#import "typst_commands.typ":*

#show: pagestyle.with(
  background_emblema: true,
  margin: (left: 4.5cm, right:3cm, top: 1.5cm, bottom: 1.0cm),
  cabecalho: false
)


#set text(
  size: 12pt
)


// Título Vertical à esquerda da página
#place(
  dx: -9.2cm,
  dy: 6cm,
  rotate(
    -90deg,
       align( 
       right,
       block(
       width:400pt,
       [
        #set text(
          size:20pt,
          fill: rgb(125,125,125)
        )
        #dados.centro
#v(-0.5cm)
        #dados.programa
#v(-0.2cm)
        ])
        )
 )
)
#place(
  dx: -1.0cm,
  dy: -0.2cm,
  line(start:(0pt,0pt), end:(0pt,600pt), stroke: 0.1pt)
  )

rEGISTRO DE ata: #dados.num_ata

#v(1.0cm)

#strong(dados.nome_aluno)

#dados.nivel

#v(1.0cm)

No dia #estilo_tempo(dados.data_defesa), às #estilo_tempo(dados.horario_defesa),
#estilo_lugar(dados.local_defesa_ou_reuniao_virtual)
reuniu-se a Banca Examinadora constituída pelos(as) professores(as) doutores(as)
#{for membro in dados.banca [ #emph(membro.nome),]}
para examinar a #emph(dados.nivel)
intitulada "#estilo_nome(dados.titulo)",
apresentada por #estilo_nome(dados.nome_aluno)
ao #emph(dados.programa) da PUC-Rio,
como requisito parcial para obtenção do grau de #emph(dados.titulo_obtido),
na área de #emph(dados.area).

#v(1cm)

#set list(marker: "[ ]")
#strong([Parecer da Banca])

- #emph(dados.nivel) aprovada
- #emph(dados.nivel) aprovada, sugerindo a incorporação,
   na versão definitiva, de observações feitas pelos examinadores
- Aprovação final da #emph(dados.nivel) condicionada ao 
   cumprimento das exigências feitas pelos examinadores
- #emph(dados.nivel) reprovada
  #line(length:100%, stroke:0.1pt)
- Em caso de aprovação, a banca solicita a alteração do Título do trabalho para: 
#v(-0.2cm)
  (colocar título em inglês e português)

#for _ in range(5) {
  line(length:100%, stroke:0.1pt)
  v(0.2cm)
}

#{
  let arr = ()
  for membro in dados.banca {
    if (has_key(membro, "obs") and membro.obs != "Suplente") or not has_key(membro, "obs") {
      arr.push(
        align(left,
          stack(
            spacing: 3pt,
            [#membro.nome],
            if has_key(membro,"obs") { membro.obs},
            if has_key(membro,"afiliacao") { membro.afiliacao},
              )
          )
        )
    }
   }

  set text(size:10pt)
  align(
    center,
    grid(
        columns: (auto, auto),
        row-gutter: 20pt,
        column-gutter:20pt,
        ..arr
      )
    )

}


#pagebreak()
#set page(
  margin: (left:2cm, right:2cm),
  )

#align(
  center,
  strong([CONDIÇÕES PARA A CONCESSÃO DO TÍTULO DE #upper(dados.titulo_concedido)])
)
#set list(
  marker: ("", "a)"),
  indent: 20pt
)
O Regulamento dos Programas de Pós-Graduação da PUC-Rio estabelece as seguintes exigências para a qualificação do grau de #emph(dados.titulo_obtido):

- Art.83:
  O aluno de Mestrado ou Doutorado não poderá defender sua dissertação ou tese sem
  ter cumprido as exigências expressas no artigo 55, alíneas a), b) e c), no caso de Mestrado, ou no artigo 56, alíneas a), b), c) e d), no caso de Doutorado.

- #if dados.titulo_concedido == "Doutor" [Art.85] else [ Art.84 ]: 
  Cumpridas as demais exigências regimentais, são condicões para requerer a concessão do 
  título de #emph(dados.titulo_concedido):
  - apresentar, defender e ser aprovado em #emph(dados.nivel), e
  - entregar ao Departamento ao qual o Programa está vinculado uma cópia  na versão eletrônica da #emph(dados.nivel), em sua forma definitiva, de acordo com as 'Normas para Apresentação de Teses e Dissertações', incorporando, se for o caso, as modificações exigidas, e devidamente aprovadas, pela Comissão Julgadora, obedecido o prazo estabelecido no artigo 72 deste Regulamento.

#align(
  center,
  strong([ENTREGA DA VERSÃO DEFINITIVA DA #dados.nivel_cab])
)
A versão final da #emph(dados.nivel), aprovada, deverá ser entregue à Coordenação do Programa em sua forma definitiva de acordo com as Normas para Apresentação de Teses de Dissertações da PUC-Rio e de acordo com o artigo 72 do Regulamento dos Programas de Pós-Graduação, conforme transcrito a seguir:

- Art.72: A versão final da Tese ou Dissertação aprovada deverá ser entregue à Coordenação do Programa no prazo máximo de um mês, quando aprovada ou aprovada com sugestões e, de até seis meses, quando a aprovação final for condicionada ao cumprimento de exigência, a partir da data da defesa;
  
- Parágrafo único. Na hipótese da Comissão Julgadora da tese ou dissertação condicionar a aprovação do trabalho à apresentação de modificações e/ou correções, caberá à mesma Comissão estabelecer o prazo para apresentação, pelo candidato ao respectivo título, das alterações exigidas, observando-se o limite máximo de seis meses. As alterações deverão ser examinadas pelos membros da Comissão, sendo a aprovação definida pela maioria simples através de parecer por escrito.

O não cumprimento do prazo de entrega da versão definitiva da #emph(dados.nivel) acarretará na
anulação da decisão anterior da Comissão Julgadora, perdendo o direito ao grau.

#v(0.5cm)

Declaro estar ciente das normas do regulamento

#v(1.5cm)

#assinatura(dados.nome_aluno)
