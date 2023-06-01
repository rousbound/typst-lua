package.cpath = package.cpath .. ';?.so'
print("package.cpath:", package.cpath)
local typst = require"typst"

local dados = {
	name = "Testeadjasldh",
	okokok = "aaaaaaaaaaaaaa",
	aaaa = {
		a = "a",
		b = "b",
		c = "c",
		d = {a = "hello world!"} 
	},
	
	["aprovacao"]="aprovada pelo Vice-Reitor Acadêmico",
	["area"]="Linguagens de Programação",
	["banca"]={
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Roberto Ierusalimschy",
			["obs"]="Orientador e Presidente",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["afiliacao"]="Universidade Federal do Rio Grande do Norte",
			["nome"]="Anamaria Martins Moreira",
			["titulacao"]="Doutor/Institut National Polytechnique de Grenoble"
		},
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Noemi de La Rocque Rodriguez",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Edward Hermann Haeusler",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["afiliacao"]="Universidade Federal de Minas Gerais - UFMG",
			["nome"]="Roberto da Silva Bigonha",
			["titulacao"]="Doutor/University of California"
		},
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Renato Fontoura de Gusmão Cerqueira",
			["obs"]="Suplente",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["nome"]="Luiz Henrique de Figueiredo",
			["obs"]="Suplente",
			["titulacao"]="Doutor/IMPA"
		}
	},
	["banca_defesa"]={
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Roberto Ierusalimschy",
			["obs"]="Orientador e Presidente",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["afiliacao"]="Universidade Federal do Rio Grande do Norte",
			["nome"]="Anamaria Martins Moreira",
			["titulacao"]="Doutor/Institut National Polytechnique de Grenoble"
		},
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Noemi de La Rocque Rodriguez",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["afiliacao"]="PUC-Rio",
			["nome"]="Edward Hermann Haeusler",
			["titulacao"]="Doutor/PUC-Rio"
		},
		{
			["afiliacao"]="Universidade Federal de Minas Gerais - UFMG",
			["nome"]="Roberto da Silva Bigonha",
			["titulacao"]="Doutor/University of California"
		}
	},
	["centro"]="Centro Técnico-Científico",
	["coord_oficio"]="Coordenador(a) Setorial de Pós-Graduação do CTC",
	["coord_pos"]="COORDENAÇÃO DE PÓS-GRADUAÇÃO E PESQUISA - CTC",
	["data_comunicacao"]="18/01/2017",
	["data_defesa"]="10 de setembro de 2004",
	["data_designacao"]="11 de agosto de 2004",
	["data_oficio"]="18/01/2017",
	["data_proposta"]="11 de agosto de 2004",
	["data_resultado"]="18/01/2017",
	["depto"]="Departamento de Informática",
	["designador"]="Coordenador Central de Pós-Graduação",
	["email_aluno"]="analuciadm@gmail.com",
	["horario_defesa"]="10:00",
	["local_defesa_ou_reuniao_virtual"]="Sala 511 do prédio RDC",
	["nivel"]="Tese de Doutorado",
	["nivel_cab"]="TESE DE DOUTORADO",
	["nome_aluno"]="Ana Lúcia de Moura",
	["num_ata"]="CTC-000/2004",
	["num_designacao"]="762/08/2014-B",
	["num_oficio"]="OF.CSPG.324/2004-B",
	["programa"]="Programa de Pós-Graduação em Informática",
	["resultado"]="Aprovado(a)",
	["resumo"]="O conceito de co-rotinas,introduzido na década de 60, constitui uma dasprimeiras propostas de uma abstração genérica de controle. Apesar dessaabstração ter sido bastante explorada para a implementação de diversoscomportamentos interessantes, são raros os exemplos de linguagens que oferecem co-rotinas.Além da ausência de uma definição precisa para o conceito, a introdução decontinuações de primeira classe e a adoção do modelo de multithreading como um padrâo para a programação concorrente muito contribuiram para o virtual abandono do interesse em co-rotinas.Esse trabalho defende o resgate de co-rotinas como uma abstração de controlepoderosa e conveniente, que pode substituir tanto continuações como threads com um conceito único e muito mais simples. Para suprir a ausência de uma definição precisa, propomos um sistema para a classificação de co-rotinas, e introduzimos o conceito de co-rotinas completas, para o qual provemos uma descrição formal, baseada em uma semântica operacional. Demostramos a seguir a equivalência de poder expressivo entre co-rotinas completas e continuaçôes one-shot, e discutimos as vantagens do uso de co-rotinas completas assimétricas ao invés deco-rotinas simétricas ou continuações, sejam elas tradicionais ou parciais.Apresentamos também exemplos de implementação de diferentes estruturas de controle com co-rotinas completas assimétricas.Finalmente, propomos um sistema de classificação para modelos de concorrência, discutindo os problemas e benefícios relacionados a cada tipo de modelo. Exploramos então o uso de co-rotinas para o suporte ao desenvolvimento de programas concorrentes baseados em modelos de gerência cooperativa de tarefas e em orientação a eventos.",
	["tipo_doc"]="tese",
	["titulo"]="Revisitando co-rotinas",
	["titulo_concedido"]="Doutor",
	["titulo_obtido"]="Doutor em Ciências - Informática"
}
local compiler = typst.compiler(".") -- Seta o root do World
local pdf_bytes, err = compiler:compile("test1.typ", dados)
if pdf_bytes then 
	local fh, err = io.open("test1_result.pdf", "wb")


	fh:write(pdf_bytes)
	fh:close()
else 
	print("Erro :", err)
end

--local pdf_bytes, err = compiler:compile("tests/test2.typ", dados)

--print("Erro :", err)

--local fh, err = io.open("tests/test2_result.pdf", "wb")

--fh:write(pdf_bytes)

--local pdf_bytes, err = compiler:compile("tests/test3.typ", dados)

--print("Erro :", err)


--local fh, err = io.open("tests/test3_result.pdf", "wb")

--fh:write(pdf_bytes)
