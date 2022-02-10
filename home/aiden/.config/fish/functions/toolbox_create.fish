function toolbox_create -a name --wraps='toolbox create'
    if not_empty $name
        toolbox create $name
        toolbox run -c $name sudo dnf install fish hub @development-tools cmake -y
    else
        toolbox create
        toolbox run sudo dnf install fish hub @development-tools cmake -y
    end
end
